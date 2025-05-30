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

#include "util/slip.hpp"

namespace StandaertHA::Util::SLIP {

  namespace {

    constexpr const uint8_t ESC = 0xDB;
    constexpr const uint8_t ESC_END = 0xDC;
    constexpr const uint8_t ESC_ESC = 0xDD;

  }

  bool encode(const uint8_t * const in_buf,
              const size_t in_size,
              uint8_t * const out_buf,
              const size_t out_buf_size,
              size_t& out_size) noexcept
  {
    size_t pos = 0;
    if (pos >= out_buf_size) {
      return false;
    }
    out_buf[pos++] = END;

    for (size_t in_pos = 0; in_pos < in_size; ++in_pos) {
      const uint8_t b = in_buf[in_pos];
      if (b == END) {
        if (pos >= out_buf_size) {
          return false;
        }
        out_buf[pos++] = ESC;
        if (pos >= out_buf_size) {
          return false;
        }
        out_buf[pos++] = ESC_END;
      } else if (b == ESC) {
        if (pos >= out_buf_size) {
          return false;
        }
        out_buf[pos++] = ESC;
        if (pos >= out_buf_size) {
          return false;
        }
        out_buf[pos++] = ESC_ESC;
      } else {
        if (pos >= out_buf_size) {
          return false;
        }
        out_buf[pos++] = b;
      }
    }

    if (pos >= out_buf_size) {
      return false;
    }
    out_buf[pos++] = END;

    out_size = pos;
    return true;
  }

  bool decode(const uint8_t * const in_buf,
              const size_t in_size,
              uint8_t * const out_buf,
              const size_t out_buf_size,
              size_t& out_size) noexcept
  {
    size_t in_pos = 1;
    size_t pos = 0;
    while (in_pos < in_size &&
           in_buf[in_pos] != END) {
      if (in_buf[in_pos] == ESC) {
        ++in_pos;
        if (in_buf[in_pos] == ESC_END) {
          if (pos >= out_buf_size) {
            return false;
          }
          out_buf[pos++] = END;
        } else if (in_buf[in_pos] == ESC_ESC) {
          if (pos >= out_buf_size) {
            return false;
          }
          out_buf[pos++] = ESC;
        } else {
          // Invalid escape
          return false;
        }
      } else {
        if (pos >= out_buf_size) {
          return false;
        }
        out_buf[pos++] = in_buf[in_pos];
      }
      ++in_pos;
    }
    out_size = pos;
    return true;
  }

}
