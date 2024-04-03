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

namespace StandaertHA {

  constexpr inline int getBit(const uint32_t i, const uint8_t bit) {
    return ((i >> bit) & 1) ? HIGH : LOW;
  }

  constexpr inline void setBit(uint32_t &i, const uint8_t bit, int v) {
    uint32_t mask = ((uint32_t)1) << bit;
    if (v == HIGH) {
      i = i | mask;
    } else {
      i = i & ~mask;
    }
  }

}
