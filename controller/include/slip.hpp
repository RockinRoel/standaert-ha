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

constexpr const uint8_t SLIP_END = 0xC0;

size_t slip_encode(const uint8_t * const in_buf,
                   const size_t in_size,
                   uint8_t * const out_buf,
                   const size_t out_size);

size_t slip_decode(const uint8_t * const in_buf,
                   const size_t in_size,
                   uint8_t * const out_buf,
                   const size_t out_size);

}
