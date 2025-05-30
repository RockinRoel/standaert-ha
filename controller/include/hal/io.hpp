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

#include "collections/bitset32.hpp"

#include <stdint.h>

namespace StandaertHA {
  struct State;

  namespace HAL::IO {

    constexpr const uint8_t NB_INPUTS = UINT8_C(32);
    constexpr const uint8_t NB_OUTPUTS = UINT8_C(32);

    /**
     * Reset all IO expanders (e.g. on bootup)
     */
    extern void toggle_resets() noexcept;

    extern void configure_inputs() noexcept;

    extern void configure_outputs() noexcept;

    extern Collections::BitSet32 read_inputs() noexcept;

    // Debounce
    extern void update_inputs(State& state) noexcept;

    extern void write_outputs(const Collections::BitSet32& state) noexcept;

  } // HAL::IO

} // StandaertHA
