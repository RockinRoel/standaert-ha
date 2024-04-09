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

namespace StandaertHA::Util::SLIP {

  constexpr const uint8_t END = 0xC0;

  [[nodiscard]] size_t encode(const uint8_t * const in_buf,
                                   const size_t in_size,
                                   uint8_t * const out_buf,
                                   const size_t out_size) noexcept;

  [[nodiscard]] size_t decode(const uint8_t * const in_buf,
                                   const size_t in_size,
                                   uint8_t * const out_buf,
                                   const size_t out_size) noexcept;

} // StandaertHA::Util
