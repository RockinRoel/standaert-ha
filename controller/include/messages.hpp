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

#include <avr/pgmspace.h>

namespace StandaertHA::Messages {
  const char MAXIMUM_CODE_SIZE_ERROR[] PROGMEM = {"Maximum code size exceeded!"};
  const char UNEXPECTED_PROGRAM_END_ERROR[] PROGMEM = {"Unexpected program end message"};
  const char UNEXPECTED_PROGRAM_DATA_ERROR[] PROGMEM = {"Unexpected program data message"};
  const char CODE_SIZE_MISMATCH_ERROR[] PROGMEM = {"Code size does not match size declared in program header!"};
  const char UNKNOWN_INSTRUCTION[] PROGMEM = {"Unknown instruction"};
  const char PREVIOUS_BYTE_ERROR[] PROGMEM = {"Previous byte is not first byte"};
  const char END_OF_PROGRAM[] PROGMEM = {"Reached end of program"};
  const char PROGRAM_VERIFICATION_ERROR[] PROGMEM = {"Program CRC check failed!"};
}
