// Copyright (C) Roel Standaert
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#include <Arduino.h>
#include <Wire.h>

#include "button_event.hpp"
#include "command.hpp"
#include "comm.hpp"
#include "constants.hpp"
#include "inet.hpp"
#include "slip.hpp"

#include "shal/interpreter.hpp"

namespace StandaertHA {

/**
 * Contains the state
 */
struct State {
  /**
   * Output state, as an array of 32 bits, 1 for HIGH, 0 for LOW
   */
  uint32_t output = 0;

  /**
   * Serial state (input buffer)
   */
  struct Serial {
    uint8_t input_pos = 0;
    uint8_t input_buffer[Comm::MAX_MESSAGE_LENGTH * 2 + 2]; // message is max 128 bytes, times 2 to account for escapes, SLIP_END twice is 2 bytes
  } serial;

  /**
   * Debounced inputs
   */
  struct DebouncedInput {
    /**
     * Current committed state (after debouncing)
     *
     * Array of 32 bits, 1 for HIGH, 0 for LOW
     *
     * Inputs are pulled low when button press starts,
     * so 0 means the button is pressed, 1 means the
     * button is not pressed.
     */
    uint32_t committed = 0xFFFFFFFF;

    /**
     * Last measured state
     *
     * Array of 32 bits, 1 for HIGH, 0 for LOW
     */
    uint32_t last      = 0xFFFFFFFF;

    /**
     * Timestamp when the last state was first measured
     */
    unsigned long timestamps[32];
  } input;

  /**
   * Need to send full state on next loop (refresh)
   */
  bool refresh = true;

  struct UploadState {
    bool uploading = false;
    uint16_t position = 0;
  } uploadState;

  Shal::Interpreter::Program program;
} state;

enum class Mode : byte {
  SERIAL_ONLY = 0,
  DEFAULT_PROGRAM = 1
};

Mode mode()
{
  return digitalRead(MODE_PIN) == HIGH ? Mode::DEFAULT_PROGRAM : Mode::SERIAL_ONLY;
}

/**
 * Reset all IO expanders (e.g. on bootup)
 */
void toggleResets()
{
  digitalWrite(RST_IN1_PIN, LOW);
  digitalWrite(RST_IN2_PIN, LOW);
  digitalWrite(RST_OUT1_PIN, LOW);
  digitalWrite(RST_OUT2_PIN, LOW);

  delay(50);

  digitalWrite(RST_IN1_PIN, HIGH);
  digitalWrite(RST_IN2_PIN, HIGH);
  digitalWrite(RST_OUT1_PIN, HIGH);
  digitalWrite(RST_OUT2_PIN, HIGH);

  delay(50);
}

void configureInputs()
{
  for (auto inAddr = IN1_ADDR; inAddr <= IN2_ADDR; ++inAddr) {
    // Set all pins as input
    Wire.beginTransmission(inAddr);
    Wire.write(MCP23017_IODIRA);
    Wire.write(B11111111);
    Wire.write(B11111111);
    Wire.endTransmission();

    // Enable pullup on all pins
    Wire.beginTransmission(inAddr);
    Wire.write(MCP23017_GPPUA);
    Wire.write(B11111111);
    Wire.write(B11111111);
    Wire.endTransmission();
  }
}

uint32_t readInputs()
{
  uint32_t result = 0;
  byte i = 0;
  for (auto inAddr = IN1_ADDR; inAddr <= IN2_ADDR; ++inAddr) {
    Wire.beginTransmission(inAddr);
    Wire.write(MCP23017_GPIOA);
    Wire.endTransmission();
    Wire.requestFrom(inAddr, 2);
    result = result | (((uint32_t)Wire.read()) << (i * 8));
    ++i;
    result = result | (((uint32_t)Wire.read()) << (i * 8));
    ++i;
  }
  return result;
}

void configureOutputs()
{
  for (auto outAddr = OUT1_ADDR; outAddr <= OUT2_ADDR; ++outAddr) {
    Wire.beginTransmission(outAddr);
    Wire.write(MCP23017_IODIRA);
    Wire.write(0x00);
    Wire.write(0x00);
    Wire.endTransmission();

    Wire.beginTransmission(outAddr);
    Wire.write(MCP23017_GPIOA);
    Wire.write(0x00);
    Wire.write(0x00);
    Wire.endTransmission();
  }
}

void writeOutputs(uint32_t state)
{
  byte i = 0;
  for (auto outAddr = OUT1_ADDR; outAddr <= OUT2_ADDR; ++outAddr) {
    Wire.beginTransmission(outAddr);
    Wire.write(MCP23017_GPIOA);
    Wire.write((state >> (i * 8)) & 0xFF);
    ++i;
    Wire.write((state >> (i * 8)) & 0xFF);
    ++i;
    Wire.endTransmission();
  }
}

void getButtonEvents(ButtonEvent * const events)
{
  const uint32_t inputs = readInputs();
  ButtonEvent *nextEvent = events;
  const unsigned long now = millis();

  for (byte i = 0; i < 32; ++i) {
    const int cur_b = getBit(inputs, i);
    const int prev_b = getBit(state.input.last, i);
    const int com_b = getBit(state.input.committed, i);
    if (cur_b != com_b &&
        cur_b == prev_b) {
      if (now - state.input.timestamps[i] >= DEBOUNCE_TIME_MILLIS) {
        setBit(state.input.committed, i, cur_b);
        *nextEvent = ButtonEvent(i, cur_b == LOW ? ButtonEvent::Type::PressStart : ButtonEvent::Type::PressEnd);
        ++nextEvent;
      }
    } else if (cur_b != com_b &&
               cur_b != prev_b) {
      state.input.timestamps[i] = now;
    }
  }

  state.input.last = inputs;
}

void transmitEvents(const ButtonEvent * const events, uint8_t nbEvents)
{
  Comm::UpdateMsg updateMsg;
  updateMsg.outputs = htonl(state.output);
  for (uint8_t i = 0; i < nbEvents; ++i) {
    updateMsg.events[i] = events[i];
  }

  Comm::Message message(updateMsg, nbEvents);
  byte buffer[Comm::MAX_MESSAGE_LENGTH];
  byte size = message.toBuffer(buffer, sizeof(buffer));
  if (size == 0) {
    // TODO(Roel): failure?
    return;
  }

  byte encodedBuf[sizeof(buffer) * 2 + 2];
  size_t encodedSize = slip_encode(buffer, size, encodedBuf, sizeof(encodedBuf));

  Serial.write(encodedBuf, encodedSize);
}

bool receive(Comm::Message& message)
{
  enum class RxState {
    SCAN, // Looking for first SLIP_END
    READ  // Reading message
  };
  static RxState s = RxState::SCAN;

  while (Serial.available() > 0) {
    int b = Serial.read();
    if (s == RxState::SCAN) {
      if (b == SLIP_END) {
        state.serial.input_buffer[0] = SLIP_END;
        state.serial.input_pos = 1;
        s = RxState::READ;
      }
    } else {
      // TODO(Roel): buffer issue?
      if (state.serial.input_pos == sizeof(state.serial.input_buffer) - 1) {
        // Buffer full, should not happen, reset
        s = RxState::SCAN;
        state.serial.input_pos = 0;
        continue;
      }
      state.serial.input_buffer[state.serial.input_pos++] = b;
      if (b == SLIP_END) {
        if (state.serial.input_pos >= 5) { // SLIP_END (1) + DATA (MIN 1) + CRC (2) + SLIP_END (1)
          byte decoded_buf[Comm::MAX_MESSAGE_LENGTH];
          size_t decoded_size = slip_decode(state.serial.input_buffer,
                                            state.serial.input_pos,
                                            decoded_buf,
                                            sizeof(decoded_buf));
          message = Comm::Message::fromBuffer(decoded_buf, decoded_size);
          s = RxState::SCAN;
          state.serial.input_pos = 0;
          return true;
        } else {
          // discard!
        }
        s = RxState::SCAN;
        state.serial.input_pos = 0;
        return false;
      }
    }
  }
  return false;
}

void handle(Comm::Message& message) {
  switch (message.type()) {
    case Comm::MsgType::Command: {
      const auto& commandMsg = message.bodyAsCommandMsg();
      for (uint8_t i = 0; i < message.bodyLength(); ++i) {
        const Command& command = commandMsg.command[i];
        if (command.type() == Command::Type::Refresh) {
          state.refresh = true;
        } else {
          state.output = command.apply(state.output);
        }
      }
      break;
    }
    case Comm::MsgType::ProgramStart: {
      state.uploadState.uploading = true;
      state.uploadState.position = 0;
      memcpy(&state.program.header(), &message.bodyAsProgramStart().header, sizeof(Shal::Interpreter::ProgramHeader));
      break;
    }
    case Comm::MsgType::ProgramData:
    case Comm::MsgType::ProgramEnd: {
      if (!state.uploadState.uploading) {
        // Wrong state
        break;
      }
      uint32_t newSize = static_cast<uint32_t>(state.uploadState.position) + static_cast<uint32_t>(message.bodyLength());
      if (newSize > static_cast<uint32_t>(Shal::Interpreter::MAX_CODE_SIZE) ||
          newSize > static_cast<uint32_t>(state.program.header().length())) {
        // Error: abort upload
        state.uploadState.uploading = false;
        state.uploadState.position = 0;
        // reload program from EEPROM
        state.program.load();
        break;
      }
      memcpy(state.program.code() + state.uploadState.position, message.bodyAsProgramData().code, message.bodyLength());
      state.uploadState.position += message.bodyLength();
      if (message.type() == Comm::MsgType::ProgramEnd) {
        // Upload done
        state.uploadState.uploading = false;
        state.uploadState.position = 0;
        if (newSize != static_cast<uint32_t>(state.program.header().length()) ||
            !state.program.verify()) {
          // Error: reload from EEPROM
          state.program.load();
          break;
        }
        // Upload done, save
        state.program.save();
      }
      break;
    }
    default: {
      // Do nothing
    }
  }
}

}

void setup() {
  using namespace StandaertHA;

  Serial.begin(115200);

  pinMode(MODE_PIN, INPUT);
  pinMode(MSB_PIN, INPUT);
  pinMode(LED_BUILTIN, OUTPUT);

  pinMode(RST_IN1_PIN, OUTPUT);
  pinMode(RST_IN2_PIN, OUTPUT);
  pinMode(RST_OUT1_PIN, OUTPUT);
  pinMode(RST_OUT2_PIN, OUTPUT);

  toggleResets();

  Wire.begin();

  configureInputs();
  configureOutputs();

  const uint32_t inputs = readInputs();
  state.input.committed = inputs;
  state.input.last = inputs;
  unsigned long t = millis();
  for (unsigned long & timestamp : state.input.timestamps) {
    timestamp = t;
  }

  state.program.load();
}

/**
 * Loop:
 *  - read inputs, get events
 *  - if MODE is 1:
 *    - process events
 *    - do postprocessing (e.g. ORring certain outputs)
 *  - transmit events and current output state over serial
 *  - receive and process commands over serial, until none are left
 *  - if MODE is 1:
 *    - do postprocessing (e.g. ORring certain outputs)
 */
void loop() {
  using namespace StandaertHA;

  const Mode m = mode();
  const uint32_t output_before = state.output;

  uint32_t input_before = state.input.last;

  ButtonEvent events[33];
  for (auto & event : events) {
    event = ButtonEvent();
  }
  getButtonEvents(events);

  if (m == Mode::DEFAULT_PROGRAM && !state.uploadState.uploading) {
    const Shal::Interpreter::FixedBitSet inputOld(input_before);
    const Shal::Interpreter::FixedBitSet inputNew(state.input.last);
    const Shal::Interpreter::FixedBitSet outputOld(state.output);
    Shal::Interpreter::VmState vmState(inputOld, inputNew, outputOld);
    bool success = state.program.cycle(vmState);
    digitalWrite(LED_BUILTIN, success ? LOW : HIGH);
    state.output = vmState.outputNew_.value();
  }

  Comm::Message message;
  if (receive(message)) {
    handle(message);
  }

  if (output_before != state.output) {
    writeOutputs(state.output);
  }

  if (state.refresh ||
      events[0].valid() ||
      output_before != state.output) {
    uint8_t nbEvents = 0;
    while (events[nbEvents].valid()) {
      ++nbEvents;
    }
    transmitEvents(events, nbEvents);
    state.refresh = false;
  }
}
