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

#include <stdint.h>

namespace StandaertHA::Util::CRC {

  [[nodiscard]] extern uint16_t calc_crc(const uint8_t* buffer, const uint16_t length) noexcept;
  [[nodiscard]] extern uint16_t update_crc(uint16_t crc_before, const uint8_t* buffer, const uint16_t length) noexcept;

}
