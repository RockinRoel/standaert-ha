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

  class BitStack32 {
  private:
    uint32_t stack_;
    uint8_t stackDepth_;

  public:
    constexpr BitStack32() noexcept
      : stack_(UINT32_C(0)),
        stackDepth_(UINT32_C(0)) {}

   // NOLINTNEXTLINE(bugprone-easily-swappable-parameters)
    constexpr explicit BitStack32(uint32_t value, uint8_t stackDepth) noexcept
      : stack_(value),
        stackDepth_(stackDepth) {}

    constexpr BitStack32(const BitStack32 &other) noexcept = default;

    constexpr BitStack32 &operator=(const BitStack32 &other) noexcept = default;

    constexpr BitStack32(BitStack32 &&) noexcept = default;

    constexpr BitStack32 &operator=(BitStack32 &&) noexcept = default;

    ~BitStack32() = default;

    constexpr void push(bool value) noexcept {
      if (value) {
        stack_ |= UINT32_C(1) << stackDepth_;
      } else {
        stack_ &= ~(UINT32_C(1) << stackDepth_);
      }
      ++stackDepth_;
    }

    constexpr bool pop() noexcept {
      const auto result = peek();
      --stackDepth_;
      return result;
    }

    [[nodiscard]] constexpr bool peek() const noexcept {
      return (stack_ & (UINT32_C(1) << (stackDepth_ - 1))) != UINT32_C(0);
    }

    [[nodiscard]] constexpr bool all_one() const noexcept {
      if (stackDepth_ == 0) {
        return true;
      }
      const uint32_t mask = UINT32_C(0xFFFF'FFFF) >> (32 - stackDepth_);
      return (stack_ & mask) == mask;
    }
  };

  // NOLINTBEGIN(cppcoreguidelines-avoid-magic-numbers)
  static_assert(BitStack32().all_one());
  static_assert(BitStack32(UINT32_C(0xFFFF'FFFF), 32).all_one());
  static_assert(BitStack32(UINT32_C(0x0000'0003), 2).all_one());
  static_assert(BitStack32(UINT32_C(0x0000'0003), 1).all_one());
  static_assert(!BitStack32(UINT32_C(0x0000'0003), 3).all_one());
  // NOLINTEND(cppcoreguidelines-avoid-magic-numbers)

}
