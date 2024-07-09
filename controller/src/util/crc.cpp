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

#include "util/crc.hpp"

#include <util/crc16.h>

namespace StandaertHA::Util::CRC {

  uint16_t calc_crc(const uint8_t * const buffer, const uint16_t length) noexcept
  {
    return update_crc(0, buffer, length);
  }

  uint16_t update_crc(const uint16_t crc_before, const uint8_t * const buffer, const uint16_t length) noexcept
  {
    uint16_t crc = crc_before;
    for (auto i = UINT16_C(0); i < length; ++i) {
      crc = _crc_xmodem_update(crc, buffer[i]);
    }
    return crc;
  }

}
