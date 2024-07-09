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

#include "hal/io.hpp"

#include "constants.hpp"
#include "state.hpp"

#include <Arduino.h>
#include <Wire.h>

namespace StandaertHA::HAL::IO {

  constexpr const long RESET_DELAY = 50L;

  /**
   * Reset all IO expanders (e.g. on bootup)
   */
  void toggle_resets() noexcept
  {
    digitalWrite(Constants::RST_IN1_PIN, LOW);
    digitalWrite(Constants::RST_IN2_PIN, LOW);
    digitalWrite(Constants::RST_OUT1_PIN, LOW);
    digitalWrite(Constants::RST_OUT2_PIN, LOW);

    delay(RESET_DELAY);

    digitalWrite(Constants::RST_IN1_PIN, HIGH);
    digitalWrite(Constants::RST_IN2_PIN, HIGH);
    digitalWrite(Constants::RST_OUT1_PIN, HIGH);
    digitalWrite(Constants::RST_OUT2_PIN, HIGH);

    delay(RESET_DELAY);
  }

  void configure_inputs() noexcept
  {
    for (auto inAddr = Constants::IN1_ADDR; inAddr <= Constants::IN2_ADDR; ++inAddr) {
      // Set all pins as input
      Wire.beginTransmission(inAddr);
      Wire.write(Constants::MCP23017_IODIRA);
      Wire.write(B11111111);
      Wire.write(B11111111);
      Wire.endTransmission();

      // Enable pullup on all pins
      Wire.beginTransmission(inAddr);
      Wire.write(Constants::MCP23017_GPPUA);
      Wire.write(B11111111);
      Wire.write(B11111111);
      Wire.endTransmission();
    }
  }

  void configure_outputs() noexcept
  {
    for (auto outAddr = Constants::OUT1_ADDR; outAddr <= Constants::OUT2_ADDR; ++outAddr) {
      Wire.beginTransmission(outAddr);
      Wire.write(Constants::MCP23017_IODIRA);
      Wire.write(0x00);
      Wire.write(0x00);
      Wire.endTransmission();

      Wire.beginTransmission(outAddr);
      Wire.write(Constants::MCP23017_GPIOA);
      Wire.write(0x00);
      Wire.write(0x00);
      Wire.endTransmission();
    }
  }

  Collections::BitSet32 read_inputs() noexcept
  {
    uint32_t result = 0;
    byte i = 0;
    for (auto inAddr = Constants::IN1_ADDR; inAddr <= Constants::IN2_ADDR; ++inAddr) {
      Wire.beginTransmission(inAddr);
      Wire.write(Constants::MCP23017_GPIOA);
      Wire.endTransmission();
      Wire.requestFrom(inAddr, 2);
      result = result | (((uint32_t)Wire.read()) << (static_cast<uint32_t>(i) * Constants::BYTE_SIZE));
      ++i;
      result = result | (((uint32_t)Wire.read()) << (static_cast<uint32_t>(i) * Constants::BYTE_SIZE));
      ++i;
    }
    return Collections::BitSet32(result);
  }

  void update_inputs(State& state) noexcept
  {
    state.input.previous = state.input.current;

    const Collections::BitSet32 inputs = HAL::IO::read_inputs();
    const unsigned long now = millis();

    for (byte i = 0; i < HAL::IO::NB_INPUTS; ++i) {
      const bool read_value = inputs.get(i);
      const bool last_read_value = state.input.last_read.get(i);
      const bool current_committed_value = state.input.current.get(i);
      const bool changed = read_value != current_committed_value;
      if (changed) {
        const bool stable = read_value == last_read_value;
        if (stable) {
          if (now - state.input.timestamps[i] >= Constants::DEBOUNCE_TIME_MILLIS) {
            state.input.current.set(i, read_value);
          }
        } else {
          state.input.timestamps[i] = now;
        }
      }
    }

    state.input.last_read = inputs;
  }

  void write_outputs(const Collections::BitSet32& state) noexcept
  {
    const auto value = state.value();
    byte i = 0;
    for (auto outAddr = Constants::OUT1_ADDR; outAddr <= Constants::OUT2_ADDR; ++outAddr) {
      Wire.beginTransmission(outAddr);
      Wire.write(Constants::MCP23017_GPIOA);
      Wire.write((value >> (static_cast<uint32_t>(i) * Constants::BYTE_SIZE)) & Constants::BYTE_MASK);
      ++i;
      Wire.write((value >> (static_cast<uint32_t>(i) * Constants::BYTE_SIZE)) & Constants::BYTE_MASK);
      ++i;
      Wire.endTransmission();
    }
  }

} // StandaertHA::HAL::IO
