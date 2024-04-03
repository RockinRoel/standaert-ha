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

#include "util.hpp"

namespace StandaertHA {

/**
 * Command, encoded as:
 * 
 * TTTIIIII
 * 
 * TTT: command type
 * IIIII: output id
 */
class Command {
public:
  enum class Type : uint8_t {
    None   = 0x00,
    Refresh = 0x20,
    Toggle = 0x40,
    Off    = 0x80,
    On     = 0xC0
  };
  
  constexpr Command()
    : data_(0)
  { }

  constexpr explicit Command(Type type, uint8_t output)
    : data_(static_cast<uint8_t>(type) | // type
            (output & 0x1F)) // output
  { }

  constexpr Type type() const {
    return static_cast<Type>(data_ & 0xE0);
  }

  constexpr uint8_t output() const {
    return data_ & 0x1F;
  }

  constexpr uint8_t raw() const {
    return data_;
  }

  constexpr static Command fromRaw(uint8_t data) {
    return Command(data);
  }

  constexpr uint32_t apply(uint32_t before) const {
    if (type() == Type::Toggle) {
      return before ^ (static_cast<uint32_t>(1) << output());
    } else if (type() == Type::Off) {
      return before & ~(static_cast<uint32_t>(1) << output());
    } else if (type() == Type::On) {
      return before | (static_cast<uint32_t>(1) << output());
    } else {
      return before;
    }
  }

private:
  constexpr explicit Command(uint8_t data)
    : data_(data)
  { }

  uint8_t data_;
} __attribute__((packed));

} // StandaertHA
