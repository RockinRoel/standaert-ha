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

#include "collections/bitset32.hpp"

namespace StandaertHA::Comm {

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

    constexpr Command() noexcept
      : data_(0)
    { }

    constexpr explicit Command(Type type, uint8_t output) noexcept
      : data_(static_cast<uint8_t>(type) | // type
              (output & 0x1F)) // output
    { }

    [[nodiscard]] constexpr Type type() const noexcept {
      return static_cast<Type>(data_ & 0xE0);
    }

    [[nodiscard]] constexpr uint8_t output() const noexcept {
      return data_ & 0x1F;
    }

    [[nodiscard]] constexpr uint8_t raw() const noexcept {
      return data_;
    }

    constexpr static Command from_raw(uint8_t data) noexcept {
      return Command(data);
    }

    [[nodiscard]] constexpr Collections::BitSet32 apply(const Collections::BitSet32 before) const noexcept {
      if (type() == Type::Toggle) {
        auto result = before;
        result.set(output(), !before.get(output()));
        return result;
      } else if (type() == Type::Off) {
        auto result = before;
        result.set(output(), false);
        return result;
      } else if (type() == Type::On) {
        auto result = before;
        result.set(output(), true);
        return result;
      } else {
        return before;
      }
    }

  private:
    constexpr explicit Command(uint8_t data) noexcept
      : data_(data)
    { }

    uint8_t data_;
  } __attribute__((packed));

  static_assert(sizeof(Command) == 1);

} // StandaertHA::Comm
