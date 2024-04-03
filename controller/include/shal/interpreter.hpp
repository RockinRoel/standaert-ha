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
#include "inet.hpp"

#include <Arduino.h>
#include <EEPROM.h>

namespace StandaertHA::Shal::Interpreter {
  class FixedBitSet {
  private:
    uint32_t set_;

  public:
    constexpr FixedBitSet() noexcept
      : set_(UINT32_C(0))
    { }

    constexpr explicit FixedBitSet(uint32_t value) noexcept
      : set_(value)
    { }

    constexpr FixedBitSet(const FixedBitSet& other) noexcept = default;

    constexpr FixedBitSet& operator=(const FixedBitSet& other) noexcept = default;

    FixedBitSet(FixedBitSet&&) = delete;
    FixedBitSet& operator=(FixedBitSet&&) = delete;

    constexpr void set(uint8_t bit, bool value) noexcept
    {
      if (value) {
        set_ |= (UINT32_C(1) << bit);
      } else {
        set_ &= ~(UINT32_C(1) << bit);
      }
    }

    [[nodiscard]] constexpr bool get(uint8_t bit) const noexcept
    {
      return (set_ & (UINT32_C(1) << bit)) != UINT32_C(0);
    }

    constexpr void clear() noexcept
    {
      set_ = UINT32_C(0);
    }

    [[nodiscard]] constexpr uint32_t value() const noexcept
    {
      return set_;
    }
  };

  class BitStack {
  private:
    uint32_t stack_;
    uint8_t stackDepth_;

  public:
    constexpr BitStack() noexcept
      : stack_(UINT32_C(0)),
        stackDepth_(UINT32_C(0))
    { }

    constexpr explicit BitStack(uint32_t value, uint8_t stackDepth) noexcept
      : stack_(value),
        stackDepth_(stackDepth)
    { }

    constexpr BitStack(const BitStack& other) noexcept = default;

    constexpr BitStack& operator=(const BitStack& other) noexcept = default;

    BitStack(BitStack&&) = delete;
    BitStack& operator=(BitStack&&) = delete;

    constexpr void push(bool value) noexcept
    {
      if (value) {
        stack_ |= UINT32_C(1) << stackDepth_;
      } else {
        stack_ &= ~(UINT32_C(1) << stackDepth_);
      }
      ++stackDepth_;
    }

    constexpr bool pop() noexcept
    {
      const auto result = peek();
      --stackDepth_;
      return result;
    }

    [[nodiscard]] constexpr bool peek() const noexcept
    {
      return (stack_ & (UINT32_C(1) << (stackDepth_ - 1))) != UINT32_C(0);
    }

    [[nodiscard]] constexpr bool all_one() const noexcept
    {
      if (stackDepth_ == 0) {
        return true;
      }
      const uint32_t mask = UINT32_C(0xFFFF'FFFF) >> (32 - stackDepth_);
      return (stack_ & mask) == mask;
    }
  };

  static_assert(BitStack().all_one());
  static_assert(BitStack(UINT32_C(0xFFFF'FFFF), 32).all_one());
  static_assert(BitStack(UINT32_C(0x0000'0003), 2).all_one());
  static_assert(BitStack(UINT32_C(0x0000'0003), 1).all_one());
  static_assert(!BitStack(UINT32_C(0x0000'0003), 3).all_one());

  class VmState {
  private:
    const FixedBitSet& inputOld_;
    const FixedBitSet& inputNew_;
    const FixedBitSet& outputOld_;
    FixedBitSet outputNew_;
    BitStack stack_;

  public:
    VmState(const FixedBitSet& inputOld,
            const FixedBitSet& inputNew,
            const FixedBitSet& outputOld)
      : inputOld_(inputOld),
        inputNew_(inputNew),
        outputOld_(outputOld),
        outputNew_(outputOld)
    { }

    VmState(const VmState&) = delete;
    VmState& operator=(const VmState&) = delete;
    VmState(VmState&&) = delete;
    VmState& operator=(VmState&&) = delete;

    friend class Program;
    friend void ::loop();
  };

  // Max program size is 256 bytes
  constexpr uint16_t PROGRAM_SIZE = 256;
#ifndef EEPROM_SIZE
#define EEPROM_SIZE (E2END + 1)
#endif
  static_assert(EEPROM_SIZE >= PROGRAM_SIZE);

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
      : length_(htons(length)),
        crc_(htons(crc))
    {}

    [[nodiscard]] const Magic& magic() const { return magic_; }
    [[nodiscard]] uint16_t length() const { return ntohs(length_); }
    [[nodiscard]] uint16_t crc() const { return ntohs(crc_); }

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
}
