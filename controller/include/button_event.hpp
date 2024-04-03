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

#pragma once

#include <Arduino.h>

namespace StandaertHA {

  /**
   * ButtonEvent, encoded as:
   *
   * TV0IIIII
   * ^^     ^
   * ||     \- Button id
   * |\------- Valid
   * \-------- Type
   *
   * e.g.
   * 01000010 : second button press start
   * 11000010 : second button press end
   * 00000000 : not valid
   */
  class ButtonEvent {
  public:
    enum class Type : uint8_t {
      PressStart = 0x00,
      PressEnd   = 0x80
    };

    constexpr ButtonEvent()
      : data_(0)
    { }

    constexpr explicit ButtonEvent(uint8_t button, Type type)
      : data_(static_cast<uint8_t>(type) | // type
              (button & 0x1F) | // button id
              0x40) // valid
    { }

    constexpr bool valid() const {
      return data_ & 0x40;
    }

    constexpr Type type() const {
      return static_cast<Type>(data_ & 0x80);
    }

    constexpr uint8_t button() const {
      return data_ & 0x1F;
    }

    constexpr uint8_t raw() const {
      return data_;
    }

    static constexpr ButtonEvent fromRaw(uint8_t data) {
      return ButtonEvent(data);
    }

    constexpr bool operator==(const ButtonEvent &other) const {
      return data_ == other.data_;
    }

    constexpr bool operator!=(const ButtonEvent &other) const {
      return !operator==(other);
    }

  private:
    constexpr explicit ButtonEvent(uint8_t data)
      : data_(data)
    { }

    uint8_t data_;
  } __attribute__((packed));

  static_assert(sizeof(ButtonEvent) == 1);

} // StandaertHA
