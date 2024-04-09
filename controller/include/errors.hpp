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

namespace StandaertHA::Errors {
  const char MAXIMUM_CODE_SIZE_ERROR[] PROGMEM = {"Maximum code size exceeded!"};
  const char CODE_SIZE_MISMATCH_ERROR[] PROGMEM = {"Code size does not match size declared in program header!"};
  const char PROGRAM_VERIFICATION_ERROR[] PROGMEM = {"Program CRC check failed!"};
}
