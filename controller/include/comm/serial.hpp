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

#include "state.hpp"

#include "collections/bitset32.hpp"
#include "comm/message.hpp"
#include "shal/interpreter.hpp"

namespace StandaertHA::Comm::Serial {

  extern void send(const Message& message) noexcept;
  extern void send_update(const State& state) noexcept;
  extern void send_program_start_ack(const Shal::Interpreter::ProgramHeader& header) noexcept;
  extern void send_program_end_ack(const Shal::Interpreter::ProgramHeader& header) noexcept;

  extern void send_error(const char* message, size_t size) noexcept;
  extern void send_info(const char* message, size_t size) noexcept;

  template<size_t size>
  inline void send_error(const char (&message)[size]) noexcept
  {
    send_error(message, size - 1);
  }

  template<size_t size>
  inline void send_info(const char (&message)[size]) noexcept
  {
    send_info(message, size - 1);
  }
}
