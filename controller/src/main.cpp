#include <Arduino.h>
#include <Wire.h>

#include "button_event.h"
#include "command.h"
#include "config.h"
#include "slip.h"
#include "standaert_ha.h"
#include "util.h"

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
  state.output = config.apply(state.output, event);
}

void transmit(const ButtonEvent * const events) {
  byte buf[36];
  byte pos = 0;
  for (byte i = 0; i < 32; ++i) {
    buf[pos] = events[i].raw();
    ++pos;
  }
  buf[pos++] = (state.output >> 24) & 0xFF;
  buf[pos++] = (state.output >> 16) & 0xFF;
  buf[pos++] = (state.output >> 8) & 0xFF;
  buf[pos++] = state.output & 0xFF;

  byte encoded_buf[128];
  size_t size = slip_encode(buf, pos, encoded_buf, 128);

  Serial.write(encoded_buf, size);
}

void receive() {

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
    postprocess();
  }

  if (events[0].valid() ||
      output_before != state.output) {
    transmit(events);
  }
  
  receive();

  if (m == Mode::DEFAULT_PROGRAM) {
    postprocess();
  }

  if (output_before != state.output) {
    writeOutputs(state.output);
  }
}