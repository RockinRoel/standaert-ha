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

namespace StandaertHA::Collections {

  class BitSet32 {
  private:
    uint32_t set_;

  public:
    constexpr BitSet32() noexcept
      : set_(UINT32_C(0)) {}

    constexpr explicit BitSet32(uint32_t value) noexcept
      : set_(value) {}

    constexpr BitSet32(const BitSet32 &other) noexcept = default;

    constexpr BitSet32 &operator=(const BitSet32 &other) noexcept = default;

    constexpr BitSet32(BitSet32 &&) noexcept = default;

    constexpr BitSet32 &operator=(BitSet32 &&) noexcept = default;

    ~BitSet32() = default;

    constexpr void set(uint8_t bit, bool value) noexcept {
      if (value) {
        set_ |= (UINT32_C(1) << bit);
      } else {
        set_ &= ~(UINT32_C(1) << bit);
      }
    }

    [[nodiscard]] constexpr bool get(uint8_t bit) const noexcept {
      return (set_ & (UINT32_C(1) << bit)) != UINT32_C(0);
    }

    constexpr void clear() noexcept {
      set_ = UINT32_C(0);
    }

    [[nodiscard]] constexpr uint32_t value() const noexcept {
      return set_;
    }

    constexpr bool operator==(const BitSet32& other) const noexcept {
      return set_ == other.set_;
    }

    constexpr bool operator!=(const BitSet32& other) const noexcept {
      return set_ != other.set_;
    }
  };

}
