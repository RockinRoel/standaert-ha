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

#include "bytecode.hpp"

#include "collections/bitstack32.hpp"
#include "collections/bitset32.hpp"
#include "util/inet.hpp"

#include <Arduino.h>
#include <EEPROM.h>

namespace StandaertHA::Shal::Interpreter {

  class VmState {
  private:
    using BitSet32 = Collections::BitSet32;
    using BitStack32 = Collections::BitStack32;

    const BitSet32& old_input_;
    const BitSet32& new_input_;
    const BitSet32& old_output_;
    BitSet32 new_output_;
    BitStack32 stack_;

  public:
    VmState(const BitSet32& old_input,
            const BitSet32& new_input,
            const BitSet32& old_output)
      : old_input_(old_input),
        new_input_(new_input),
        old_output_(old_output),
        new_output_(old_output)
    { }

    VmState(const VmState&) = delete;
    VmState& operator=(const VmState&) = delete;
    VmState(VmState&&) = delete;
    VmState& operator=(VmState&&) = delete;

    const BitSet32& new_output() const
    {
      return new_output_;
    }

    friend class Program;
  };

  // Max program size is 256 bytes
#ifndef EEPROM_SIZE
#define EEPROM_SIZE (E2END + 1)
#endif
  constexpr uint16_t PROGRAM_SIZE = EEPROM_SIZE < 1024 ? EEPROM_SIZE : 1024;

  class ProgramHeader {
  public:
    using Magic = uint8_t[4];

    ProgramHeader() = default;
    explicit ProgramHeader(const uint8_t* const buffer, const uint8_t buffer_length)
    {
      if (buffer_length != sizeof(*this)) {
        memset(this, 0, 8);
        return;
      }

      memcpy(this, buffer, sizeof(*this));
    }

    explicit ProgramHeader(uint16_t length, uint16_t crc)
      : length_(Util::Inet::htons(length)),
        crc_(Util::Inet::htons(crc))
    {}

    [[nodiscard]] const Magic& magic() const { return magic_; }
    [[nodiscard]] uint16_t length() const { return Util::Inet::ntohs(length_); }
    [[nodiscard]] uint16_t crc() const { return Util::Inet::ntohs(crc_); }

  private:
    Magic magic_ = {'S', 'H', 'A', 'L'};
    uint16_t length_ = UINT16_C(0);
    uint16_t crc_ = UINT16_C(0);
  } __attribute__((packed));

  static_assert(sizeof(ProgramHeader) == 8);

  constexpr uint16_t MAX_CODE_SIZE = PROGRAM_SIZE - sizeof(ProgramHeader);

  class Program {
  private:
    ProgramHeader header_;
    uint8_t code_[PROGRAM_SIZE - sizeof(ProgramHeader)] = { Bytecode::INSTR_END };

  public:
    Program();

    ProgramHeader& header() { return header_; }
    uint8_t* code() { return code_; }
    [[nodiscard]] const ProgramHeader& header() const { return header_; }

    [[nodiscard]] bool verify() const;

    void clear();

    // Save program to EEPROM
    void save();

    // Load program from EEPROM, and verify
    bool load();
    // Execute one cycle of the program,
    // returns false if it failed to execute
    // properly for some reason
    bool cycle(VmState& state) const;

  private:
    enum class Value {
      Low,
      High,
    };

    enum class IsWas {
      Was,
      Is,
    };

    enum class Edge {
      Rising,
      Falling,
    };

    enum class InOut {
      Input,
      Output,
    };

    void instrAnd(VmState& state) const;
    void instrOr(VmState& state) const;
    void instrXor(VmState& state) const;
    void instrNot(VmState& state) const;
    void instrPop(VmState& state) const;

    void instrSet(VmState& state, uint8_t output, Value value) const;
    void instrToggle(VmState& state, uint8_t output) const;
    void instrOn(VmState& state, Edge edge, uint8_t input) const;
    void instrIf(VmState& state, InOut inOut, IsWas isWas, uint8_t n, Value value) const;

    [[nodiscard]] uint16_t calcCrc() const;
  };

  static_assert(sizeof(Program) == PROGRAM_SIZE);

} // StandaertHA::SHAL::Interpreter
