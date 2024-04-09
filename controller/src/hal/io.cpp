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

#include <Wire.h>

namespace StandaertHA::HAL::IO {

  /**
   * Reset all IO expanders (e.g. on bootup)
   */
  void toggle_resets() noexcept
  {
    digitalWrite(Constants::RST_IN1_PIN, LOW);
    digitalWrite(Constants::RST_IN2_PIN, LOW);
    digitalWrite(Constants::RST_OUT1_PIN, LOW);
    digitalWrite(Constants::RST_OUT2_PIN, LOW);

    delay(50);

    digitalWrite(Constants::RST_IN1_PIN, HIGH);
    digitalWrite(Constants::RST_IN2_PIN, HIGH);
    digitalWrite(Constants::RST_OUT1_PIN, HIGH);
    digitalWrite(Constants::RST_OUT2_PIN, HIGH);

    delay(50);
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
      result = result | (((uint32_t)Wire.read()) << (i * 8));
      ++i;
      result = result | (((uint32_t)Wire.read()) << (i * 8));
      ++i;
    }
    return Collections::BitSet32(result);
  }

  void write_outputs(const Collections::BitSet32 state) noexcept
  {
    const auto value = state.value();
    byte i = 0;
    for (auto outAddr = Constants::OUT1_ADDR; outAddr <= Constants::OUT2_ADDR; ++outAddr) {
      Wire.beginTransmission(outAddr);
      Wire.write(Constants::MCP23017_GPIOA);
      Wire.write((value >> (i * 8)) & 0xFF);
      ++i;
      Wire.write((value >> (i * 8)) & 0xFF);
      ++i;
      Wire.endTransmission();
    }
  }

} // StandaertHA::HAL::IO
