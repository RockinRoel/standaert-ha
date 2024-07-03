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

namespace StandaertHA::Comm {
  constexpr uint8_t EVENT_TYPE_MASK = 0xE0;
  constexpr uint8_t INPUT_MASK = 0x1F;

  /**
   * Event, encoded as:
   *
   * TTTIIIII
   * 
   * TTT: event type
   * IIIII: input id
   *
   * e.g.
   * 00100010 : third input (input id 2, counting from 0) falling edge
   * 01100010 : third input (input id 2, counting from 0) rising edge
   */
  class Event {
  public:
    enum class Type : uint8_t {
      Uninit      = 0x00,
      FallingEdge = 0x20,
      RisingEdge  = 0x60,
    };

    constexpr Event() noexcept
      : data_(0)
    { }

    constexpr explicit Event(Type type, uint8_t input) noexcept
      : data_(static_cast<uint8_t>(type) | // type
              (input & 0x1F)) // input id
    { }

    [[nodiscard]] constexpr Type type() const noexcept {
      return static_cast<Type>(data_ & EVENT_TYPE_MASK);
    }

    [[nodiscard]] constexpr uint8_t input() const noexcept {
      return data_ & INPUT_MASK;
    }

    [[nodiscard]] constexpr uint8_t raw() const noexcept {
      return data_;
    }

    [[nodiscard]] static constexpr Event from_raw(uint8_t data) noexcept {
      return Event(data);
    }

    [[nodiscard]] constexpr bool operator==(const Event& other) const noexcept {
      return data_ == other.data_;
    }

    [[nodiscard]] constexpr bool operator!=(const Event& other) const noexcept {
      return !operator==(other);
    }

  private:
    constexpr explicit Event(uint8_t data) noexcept
      : data_(data)
    { }

    uint8_t data_;
  } __attribute__((packed));

  static_assert(sizeof(Event) == 1);

} // StandaertHA::Comm
