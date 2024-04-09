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

#include "shal/interpreter.hpp"

#include <Arduino.h>

#include <avr/eeprom.h>
#include <util/crc16.h>

namespace StandaertHA::Shal::Interpreter {

  Program::Program()
  {
    using namespace Bytecode;
    uint16_t pc = UINT16_C(0);
    for (uint8_t i = 0; i < 32; ++i) {
      code_[pc++] = FIRST_BYTE_PREFIX | INSTR_ON;
      code_[pc++] = SECOND_BYTE_PREFIX | i;
      code_[pc++] = FIRST_BYTE_PREFIX | INSTR_TOGGLE;
      code_[pc++] = SECOND_BYTE_PREFIX | (31 - i);
      code_[pc++] = INSTR_POP;
    }
    #if 0
    code_[pc++] = FIRST_BYTE_PREFIX | INSTR_IF | IF_IO_MASK | IF_IS_WAS_MASK | IF_VALUE_MASK;
    code_[pc++] = SECOND_BYTE_PREFIX | (31 - 0);
    code_[pc++] = FIRST_BYTE_PREFIX | INSTR_IF | IF_IO_MASK | IF_IS_WAS_MASK | IF_VALUE_MASK;
    code_[pc++] = SECOND_BYTE_PREFIX | (31 - 1);
    code_[pc++] = INSTR_OR;
    code_[pc++] = FIRST_BYTE_PREFIX | INSTR_SET | SET_VALUE_MASK;
    code_[pc++] = SECOND_BYTE_PREFIX | (31 - 2);
    code_[pc++] = INSTR_NOT;
    code_[pc++] = FIRST_BYTE_PREFIX | INSTR_SET;
    code_[pc++] = SECOND_BYTE_PREFIX | (31 - 2);
    code_[pc++] = INSTR_POP;
    #endif
    code_[pc++] = INSTR_END;
    header_ = ProgramHeader(pc, calcCrc());
  }

  bool Program::verify() const
  {
    const auto magicVerified =
      header_.magic()[0] == 'S' &&
      header_.magic()[1] == 'H' &&
      header_.magic()[2] == 'A' &&
      header_.magic()[3] == 'L';
    if (!magicVerified) {
      // Does not start with magic string
      return false;
    }

    if (header_.length() > sizeof(code_)) {
      // Length too long
      return false;
    }

    uint16_t crc = 0;
    for (uint16_t i = UINT16_C(0); i < header_.length(); ++i) {
      crc = _crc_xmodem_update(crc, code_[i]);
    }

    return header_.crc() == crc;
  }

  void Program::clear()
  {
    header_ = ProgramHeader();
  }

  void Program::save()
  {
    eeprom_write_block(this, nullptr, sizeof(ProgramHeader) + header_.length());
  }

  bool Program::load()
  {
    eeprom_read_block(&header_, nullptr, sizeof(header_));

    const auto magicVerified =
      header_.magic()[0] == 'S' &&
      header_.magic()[1] == 'H' &&
      header_.magic()[2] == 'A' &&
      header_.magic()[3] == 'L';

    if (!magicVerified) {
      clear();
      return false;
    }

    if (header_.length() > sizeof(code_)) {
      clear();
      return false;
    }

    eeprom_read_block(code_, static_cast<uint8_t*>(nullptr) + sizeof(ProgramHeader), header_.length());

    if (!verify()) {
      clear();
      return false;
    }

    return true;
  }

  bool Program::cycle(VmState& state) const
  {
    using namespace Bytecode;
    state.new_output_ = state.old_output_;
    uint8_t prevByte = INSTR_END;
    for (uint16_t i = UINT16_C(0); i < header_.length(); ++i) {
      uint8_t byte = code_[i];
      if (is_single_byte(byte)) {
        switch (byte & SINGLE_BYTE_MASK) {
        case INSTR_END:
          // DONE
          return true;
        case INSTR_AND:
          instrAnd(state);
          break;
        case INSTR_OR:
          instrOr(state);
          break;
        case INSTR_XOR:
          instrXor(state);
          break;
        case INSTR_NOT:
          instrNot(state);
          break;
        case INSTR_POP:
          instrPop(state);
          break;
        default:
          state.new_output_ = state.old_output_;
          return false;
        }
      } else if (is_second_byte(byte)) {
        if (!is_first_byte(prevByte)) {
          state.new_output_ = state.old_output_;
          return false;
        }
        const uint8_t value = byte & DUAL_BYTE_MASK;
        const uint8_t instr = prevByte & DUAL_BYTE_MASK;
        if ((instr & INSTR_SET_MASK) == INSTR_SET) {
          instrSet(
            state,
            value,
            (instr & SET_VALUE_MASK) != 0 ? Value::High : Value::Low
          );
        } else if ((instr & INSTR_TOGGLE_MASK) == INSTR_TOGGLE) {
          instrToggle(
            state,
            value
          );
        } else if ((instr & INSTR_ON_MASK) == INSTR_ON) {
          instrOn(
            state,
            (instr & ON_EDGE_MASK) != 0 ? Edge::Rising : Edge::Falling,
            value
          );
        } else if ((instr & INSTR_IF_MASK) == INSTR_IF) {
          instrIf(
            state,
            (instr & IF_IO_MASK) != 0 ? InOut::Output : InOut::Input,
            (instr & IF_IS_WAS_MASK) != 0 ? IsWas::Is : IsWas::Was,
            value,
            (instr & IF_VALUE_MASK) != 0 ? Value::High : Value::Low
          );
        } else {
          state.new_output_ = state.old_output_;
          return false;
        }
      } else if (!is_first_byte(byte)) {
        state.new_output_ = state.old_output_;
        return false;
      }
      prevByte = byte;
    }
    state.new_output_ = state.old_output_;
    return false;
  }

  void Program::instrAnd(VmState& state) const
  {
    auto b1 = state.stack_.pop();
    auto b2 = state.stack_.pop();
    state.stack_.push(b1 && b2);
  }

  void Program::instrOr(VmState& state) const
  {
    auto b1 = state.stack_.pop();
    auto b2 = state.stack_.pop();
    state.stack_.push(b1 || b2);
  }

  void Program::instrXor(VmState& state) const
  {
    auto b1 = state.stack_.pop();
    auto b2 = state.stack_.pop();
    state.stack_.push(b1 != b2);
  }

  void Program::instrNot(VmState& state) const
  {
    auto b = state.stack_.pop();
    state.stack_.push(!b);
  }

  void Program::instrPop(VmState& state) const
  {
    [[maybe_unused]] auto b = state.stack_.pop();
  }

  void Program::instrSet(VmState& state, uint8_t output, Value value) const
  {
    if (!state.stack_.all_one()) {
      return;
    }
    state.new_output_.set(output, value == Value::High);
  }

  void Program::instrToggle(VmState& state, uint8_t output) const
  {
    if (!state.stack_.all_one()) {
      return;
    }
    auto old = state.new_output_.get(output);
    state.new_output_.set(output, !old);
  }

  void Program::instrOn(VmState& state, Edge edge, uint8_t input) const
  {
    switch (edge) {
    case Edge::Falling: {
      auto b = state.old_input_.get(input) && !state.new_input_.get(input);
      state.stack_.push(b);
      break;
    }
    case Edge::Rising: {
      auto b = !state.old_input_.get(input) && state.new_input_.get(input);
      state.stack_.push(b);
      break;
    }
    default:
      __builtin_unreachable();
    }
  }

  void Program::instrIf(VmState& state, InOut inOut, IsWas isWas, uint8_t n, Value value) const
  {
    const Collections::BitSet32* set = nullptr;
    if (inOut == InOut::Input) {
      if (isWas == IsWas::Was) {
        set = &state.old_input_;
      } else {
        // isWas == IsWas::Is
        set = &state.new_input_;
      }
    } else {
      // inOut == InOut::Output
      if (isWas == IsWas::Was) {
        set = &state.old_output_;
      } else {
        // isWas == IsWas::Is
        set = &state.new_output_;
      }
    }
    if (value == Value::Low) {
      auto b = !set->get(n);
      state.stack_.push(b);
    } else {
      // value == Value::High
      auto b = set->get(n);
      state.stack_.push(b);
    }
  }

  uint16_t Program::calcCrc() const
  {
    if (header_.length() > sizeof(code_)) {
      return UINT16_MAX;
    }
    uint16_t crc = 0;
    for (auto i = UINT16_C(0); i < header_.length(); ++i) {
      crc = _crc_xmodem_update(crc, code_[i]);
    }
    return crc;
  }

}
