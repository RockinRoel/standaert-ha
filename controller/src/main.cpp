#include <Arduino.h>
#include <Wire.h>

#include "button_event.hpp"
#include "command.hpp"
#include "config.hpp"
#include "constants.hpp"
#include "slip.hpp"

#include <util/crc16.h>

namespace StandaertHA {

namespace {
  constexpr const byte MAX_COMMANDS = 64;
}

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
    uint8_t input_buffer[128];
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
} state;

enum class Mode : byte {
  SERIAL_ONLY = 0,
  DEFAULT_PROGRAM = 1
};

Mode mode() {
  return digitalRead(MODE_PIN) == HIGH ? Mode::DEFAULT_PROGRAM : Mode::SERIAL_ONLY;
}

/**
 * Reset all IO expanders (e.g. on bootup)
 */
void toggleResets() {
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

void configureInputs() {
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

uint32_t readInputs() {
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

void configureOutputs() {
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

void writeOutputs(uint32_t state) {
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

void getButtonEvents(ButtonEvent * const events) {
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

void processEvent(const ButtonEvent &event) {
  DSL::Context context(event, state.output);
  handleEvent(context);
  state.output = context.outputAfter();
}

void transmit(const ButtonEvent * const events) {
  byte buf[38];
  byte pos = 0;
  for (byte i = 0; i < 32; ++i) {
    buf[pos] = events[i].raw();
    ++pos;
  }
  buf[pos++] = (state.output >> 24) & 0xFF;
  buf[pos++] = (state.output >> 16) & 0xFF;
  buf[pos++] = (state.output >> 8) & 0xFF;
  buf[pos++] = state.output & 0xFF;

  uint16_t crc = 0;
  for (byte i = 0; i < pos; ++i) {
    crc = _crc_xmodem_update(crc, buf[i]);
  }

  buf[pos++] = (crc >> 8) & 0xFF;
  buf[pos++] = crc & 0xFF;

  byte encoded_buf[sizeof(buf) * 2 + 2];
  size_t size = slip_encode(buf, pos, encoded_buf, sizeof(encoded_buf));

  Serial.write(encoded_buf, size);
}

void receive(Command * const commands) {
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
      if (state.serial.input_pos == sizeof(state.serial.input_buffer) - 1) {
        // Buffer full, should not happen, reset
        s = RxState::SCAN;
        state.serial.input_pos = 0;
        continue;
      }
      state.serial.input_buffer[state.serial.input_pos++] = b;
      if (b == SLIP_END) {
        if (state.serial.input_pos >= 5) { // SLIP_END (1) + DATA (MIN 1) + CRC (2) + SLIP_END (1)
          byte decoded_buf[sizeof(state.serial.input_buffer)];
          size_t decoded_size = slip_decode(state.serial.input_buffer,
                                            state.serial.input_pos,
                                            decoded_buf,
                                            sizeof(decoded_buf));
          uint16_t crc = 0;
          for (size_t i = 0; i < decoded_size - 2; ++i) {
            crc = _crc_xmodem_update(crc, decoded_buf[i]);
          }
          uint16_t received_crc = (static_cast<uint16_t>(decoded_buf[decoded_size - 2]) << 8 |
                                   decoded_buf[decoded_size - 1]);
          if (crc == received_crc) {
            for (size_t i = 0; i < decoded_size - 2 && i < MAX_COMMANDS; ++i) {
              commands[i] = Command::fromRaw(decoded_buf[i]);
            }
          } else {
            // discard!
          }
        } else {
          // discard!
        }
        s = RxState::SCAN;
        state.serial.input_pos = 0;
        return;
      }
    }
  }
}

}

void setup() {
  using namespace StandaertHA;

  Serial.begin(9600);

  pinMode(MODE_PIN, INPUT);
  pinMode(MSB_PIN, INPUT);

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
  for (byte i = 0; i < 32; ++i) {
    state.input.timestamps[i] = t;
  }
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

  ButtonEvent events[33];
  for (int i = 0; i < 33; ++i) {
    events[i] = ButtonEvent();
  }
  getButtonEvents(events);

  if (m == Mode::DEFAULT_PROGRAM) {
    for (ButtonEvent *ev = &events[0]; ev->valid(); ++ev) {
      processEvent(*ev);
    }
    postprocess(state.output);
  }

  if (state.refresh ||
      events[0].valid() ||
      output_before != state.output) {
    transmit(events);
    state.refresh = false;
  }
  
  Command commands[MAX_COMMANDS];
  for (int i = 0; i < MAX_COMMANDS; ++i) {
    commands[i] = Command();
  }
  receive(commands);

  const uint32_t output_before_commands = state.output;
  for (int i = 0; i < MAX_COMMANDS; ++i) {
    if (commands[i].type() == Command::Type::Refresh) {
      state.refresh = true;
    } else {
      state.output = commands[i].apply(state.output);
    }
  }

  if (m == Mode::DEFAULT_PROGRAM) {
    postprocess(state.output);
  }

  if (output_before_commands != state.output) {
    state.refresh = true;
  }

  if (output_before != state.output) {
    writeOutputs(state.output);
  }
}
