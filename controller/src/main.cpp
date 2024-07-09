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

#include "constants.hpp"
#include "state.hpp"

#include "collections/bitset32.hpp"
#include "comm/serial.hpp"
#include "hal/io.hpp"

#include <Arduino.h>
#include <Wire.h>

namespace StandaertHA {
  State state;
}

void setup() {
  using namespace StandaertHA;

  // Configure serial connection
  Serial.begin(Constants::SERIAL_BAUD_RATE);

  // Configure pins
  pinMode(Constants::MODE_PIN, INPUT);
  pinMode(Constants::MSB_PIN, INPUT);
  pinMode(LED_BUILTIN, OUTPUT);
  pinMode(Constants::RST_IN1_PIN, OUTPUT);
  pinMode(Constants::RST_IN2_PIN, OUTPUT);
  pinMode(Constants::RST_OUT1_PIN, OUTPUT);
  pinMode(Constants::RST_OUT2_PIN, OUTPUT);

  // Reset IO expanders
  HAL::IO::toggle_resets();

  // Init I2C bus
  Wire.begin();

  // Configure IO expanders
  HAL::IO::configure_inputs();
  HAL::IO::configure_outputs();

  // Setup debouncer state
  const Collections::BitSet32 inputs = HAL::IO::read_inputs();
  state.input.current = inputs;
  state.input.last_read = inputs;
  unsigned long t = millis();
  for (unsigned long& timestamp : state.input.timestamps) {
    timestamp = t;
  }

  // Load program
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

  const Collections::BitSet32 output_before = state.output;

  if (Comm::Serial::receive(state)) {
    state.handle_message();
  }
  HAL::IO::update_inputs(state);
  bool success = state.run_program();
  digitalWrite(LED_BUILTIN, success ? LOW : HIGH);
  state.update_outputs(output_before);
  state.send_update(output_before);
}
