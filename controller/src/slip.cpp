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

#include "slip.hpp"

namespace StandaertHA {

namespace {

constexpr const uint8_t SLIP_ESC = 0xDB;
constexpr const uint8_t SLIP_ESC_END = 0xDC;
constexpr const uint8_t SLIP_ESC_ESC = 0xDD;

}

size_t slip_encode(const uint8_t * const in_buf,
                   const size_t in_size,
                   uint8_t * const out_buf,
                   const size_t out_size)
{
  size_t pos = 0;
  out_buf[pos++] = SLIP_END;

  for (size_t in_pos = 0; in_pos < in_size; ++in_pos) {
    const uint8_t b = in_buf[in_pos];
    if (b == SLIP_END) {
      out_buf[pos++] = SLIP_ESC;
      out_buf[pos++] = SLIP_ESC_END;
    } else if (b == SLIP_ESC) {
      out_buf[pos++] = SLIP_ESC;
      out_buf[pos++] = SLIP_ESC_ESC;
    } else {
      out_buf[pos++] = b;
    }
  }

  out_buf[pos++] = SLIP_END;
  return pos;
}

size_t slip_decode(const uint8_t * const in_buf,
                   const size_t in_size,
                   uint8_t * const out_buf,
                   const size_t out_size)
{
  size_t in_pos = 1;
  size_t pos = 0;
  while (in_pos < in_size &&
         in_buf[in_pos] != SLIP_END) {
    if (in_buf[in_pos] == SLIP_ESC) {
        ++in_pos;
        if (in_buf[in_pos] == SLIP_ESC_END) {
            out_buf[pos++] = SLIP_END;
        } else if (in_buf[in_pos] == SLIP_ESC_ESC) {
            out_buf[pos++] = SLIP_ESC;
        } else {
            // TODO: error?
        }
    } else {
        out_buf[pos++] = in_buf[in_pos];
    }
    ++in_pos;
  }
  return pos;
}

}