#include <Arduino.h>
#include <Wire.h>

#include "slip.h"
#include "standaert_ha.h"

namespace StandaertHA {

struct State {
  uint32_t output = 0;
  uint32_t committedState = 0xFFFFFFFF;
  uint32_t lastState = 0xFFFFFFFF;
  unsigned long stateTimestamps[32];
} state;

enum class Mode : byte {
  SERIAL_ONLY = 0,
  DEFAULT_PROGRAM = 1
};

Mode mode() {
  return digitalRead(MODE_PIN) == HIGH ? Mode::DEFAULT_PROGRAM : Mode::SERIAL_ONLY;
}

class ButtonEvent {
public:
  enum class Type {
    PressStart,
    PressEnd
  };

  constexpr ButtonEvent()
    : data_(0)
  { }

  constexpr ButtonEvent(byte button,
                        Type type)
    : data_((type == Type::PressStart ? B00000000 : B10000000) | (button & B00111111) | B01000000)
  { }

  constexpr ButtonEvent(const ButtonEvent &e)
    : data_(e.data_)
  { }

  ButtonEvent &operator=(const ButtonEvent &e)
  {
    data_ = e.data_;

    return *this;
  }
  
  constexpr bool valid() const {
    return data_ & B01000000;
  }

  constexpr Type type() const {
    return ((data_ & B10000000) ? Type::PressEnd : Type::PressStart);
  }

  constexpr byte button() const {
    return data_ & B00111111;
  }

  constexpr byte raw() const {
    return data_;
  }

private:
  byte data_;
};

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
    Wire.beginTransmission(inAddr);
    Wire.write(MCP23017_IODIRA);
    Wire.write(B11111111);
    Wire.write(B11111111);
    Wire.endTransmission();

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

constexpr inline int getBit(const uint32_t i, const byte bit) {
  return ((i >> bit) & 1) ? HIGH : LOW;
}

void setBit(uint32_t &i, const byte bit, int v) {
  uint32_t mask = ((uint32_t)1) << bit;
  if (v == HIGH) {
    i = i | mask;
  } else {
    i = i & ~mask;
  }
}

void getButtonEvents(ButtonEvent * const events) {
  const uint32_t inputs = readInputs();
  ButtonEvent *nextEvent = events;
  const unsigned long now = millis();

  for (byte i = 0; i < 32; ++i) {
    const int cur_b = getBit(inputs, i);
    const int prev_b = getBit(state.lastState, i);
    const int com_b = getBit(state.committedState, i);
    if (cur_b != com_b &&
        cur_b == prev_b) {
      if (now - state.stateTimestamps[i] >= DEBOUNCE_TIME_MILLIS) {
        setBit(state.committedState, i, cur_b);
        *nextEvent = ButtonEvent(i, cur_b == LOW ? ButtonEvent::Type::PressStart : ButtonEvent::Type::PressEnd);
        ++nextEvent;
      }
    } else if (cur_b != com_b &&
               cur_b != prev_b) {
      state.stateTimestamps[i] = now;
    }
  }

  state.lastState = inputs;
}

constexpr const uint32_t kitchenTableLight = 31;
constexpr const uint32_t blablaLight = 30;
constexpr const uint32_t thirdLight = 29;

constexpr uint32_t mask(const byte b)
{
  return ((uint32_t)1) << b;
}

constexpr const uint32_t pressStartToggleMatrix[32] =
  {
    0x80000000,
    0x40000000,
    0x20000000,
    0x10000000,
    0x08000000,
    0x04000000,
    0x02000000,
    0x01000000,
    0x00800000,
    0x00400000,
    0x00200000,
    0x00100000,
    0x00080000,
    0x00040000,
    0x00020000,
    0x00010000,
    0x00008000,
    0x00004000,
    0x00002000,
    0x00001000,
    0x00000800,
    0x00000400,
    0x00000200,
    0x00000100,
    0x00000080,
    0x00000040,
    0x00000020,
    0x00000010,
    0x00000008,
    0x00000004,
    0x00000002,
    0x00000001
  };

void processEvent(ButtonEvent &event) {
  if (event.type() == ButtonEvent::Type::PressStart) {
    uint32_t toggle = pressStartToggleMatrix[event.button()];
    state.output = state.output ^ toggle;
  }
}

void postprocess() {
  /*
  if (getBit(state.output, kitchenTableLight) == HIGH ||
      getBit(state.output, blablaLight) == HIGH) {
    setBit(state.output, thirdLight, HIGH);
  } else {
    setBit(state.output, thirdLight, LOW);
  }
  */
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

  state.committedState = readInputs();
  state.lastState = readInputs();
  unsigned long t = millis();
  for (byte i = 0; i < 32; ++i) {
    state.stateTimestamps[i] = t;
  }
}

void loop() {
  using namespace StandaertHA;
  const uint32_t output_before = state.output;

  ButtonEvent events[32];
  getButtonEvents(events);

  const Mode m = mode();
  if (m == Mode::DEFAULT_PROGRAM) {
    for (ButtonEvent *ev = &events[0]; ev->valid(); ++ev) {
      processEvent(*ev);
    }
    postprocess();
  }

/*
  if (events[0].valid() ||
      output_before != state.output) {
    transmit(events);
  }
  
  receive();
*/

  if (m == Mode::DEFAULT_PROGRAM) {
    postprocess();
  }

  if (output_before != state.output) {
    writeOutputs(state.output);
  }
}