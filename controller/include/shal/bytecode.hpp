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

namespace StandaertHA::Shal::Bytecode {

  constexpr uint8_t SET_VALUE_MASK     = UINT8_C(0b0000'0001U);
  constexpr uint8_t ON_EDGE_MASK       = UINT8_C(0b0000'0001U);
  constexpr uint8_t IF_IS_WAS_MASK     = UINT8_C(0b0000'0100U);
  constexpr uint8_t IF_IO_MASK         = UINT8_C(0b0000'0010U);
  constexpr uint8_t IF_VALUE_MASK      = UINT8_C(0b0000'0001U);

  constexpr uint8_t SINGLE_BYTE_MASK   = UINT8_C(0b0111'1111U);
  constexpr uint8_t DUAL_BYTE_MASK     = UINT8_C(0b0011'1111U);

  constexpr uint8_t SINGLE_BYTE_PREFIX = UINT8_C(0b0000'0000U);
  constexpr uint8_t FIRST_BYTE_PREFIX  = UINT8_C(0b1000'0000U);
  constexpr uint8_t SECOND_BYTE_PREFIX = UINT8_C(0b1100'0000U);

  constexpr uint8_t INSTR_END          = UINT8_C(0b0000'0000U);
  constexpr uint8_t INSTR_AND          = UINT8_C(0b0000'0001U);
  constexpr uint8_t INSTR_OR           = UINT8_C(0b0000'0010U);
  constexpr uint8_t INSTR_XOR          = UINT8_C(0b0000'0011U);
  constexpr uint8_t INSTR_NOT          = UINT8_C(0b0000'0100U);
  constexpr uint8_t INSTR_POP          = UINT8_C(0b0000'0101U);

  constexpr uint8_t INSTR_SET          = UINT8_C(0b0000'0000U);
  constexpr uint8_t INSTR_TOGGLE       = UINT8_C(0b0000'0010U);
  constexpr uint8_t INSTR_ON           = UINT8_C(0b0000'0100U);
  constexpr uint8_t INSTR_IF           = UINT8_C(0b0000'1000U);

  constexpr uint8_t INSTR_SET_MASK     = DUAL_BYTE_MASK & ~SET_VALUE_MASK;
  constexpr uint8_t INSTR_TOGGLE_MASK  = DUAL_BYTE_MASK;
  constexpr uint8_t INSTR_ON_MASK      = DUAL_BYTE_MASK & ~ON_EDGE_MASK;
  constexpr uint8_t INSTR_IF_MASK      = DUAL_BYTE_MASK & ~IF_IS_WAS_MASK & ~IF_IO_MASK & ~IF_VALUE_MASK;

  static_assert(INSTR_SET_MASK    == UINT8_C(0b0011'1110));
  static_assert(INSTR_TOGGLE_MASK == UINT8_C(0b0011'1111));
  static_assert(INSTR_ON_MASK     == UINT8_C(0b0011'1110));
  static_assert(INSTR_IF_MASK     == UINT8_C(0b0011'1000));

  constexpr bool is_single_byte(uint8_t byte) {
    return (byte & FIRST_BYTE_PREFIX) != FIRST_BYTE_PREFIX;
  }

  constexpr bool is_dual_byte(uint8_t byte) {
    return (byte & FIRST_BYTE_PREFIX) == FIRST_BYTE_PREFIX;
  }

  constexpr bool is_first_byte(uint8_t byte) {
    return is_dual_byte(byte) && (byte & SECOND_BYTE_PREFIX) != SECOND_BYTE_PREFIX;
  }

  constexpr bool is_second_byte(uint8_t byte) {
    return is_dual_byte(byte) && (byte & SECOND_BYTE_PREFIX) == SECOND_BYTE_PREFIX;
  }

}
