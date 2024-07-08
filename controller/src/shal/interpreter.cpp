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

#include "comm/serial.hpp"
#include "messages.hpp"

#include <avr/eeprom.h>
#include <util/crc16.h>

namespace StandaertHA::Shal::Interpreter {

  bool VmContext::run(const Program& program) noexcept
  {
    using namespace Bytecode;
    new_output_ = old_output_;
    uint8_t prevByte = INSTR_END;
    const uint8_t* code = program.code();
    for (uint16_t i = UINT16_C(0); i < program.header().length(); ++i) {
      uint8_t byte = code[i];
      if (is_single_byte(byte)) {
        switch (byte & SINGLE_BYTE_MASK) {
          case INSTR_END:
            // DONE
            return true;
          case INSTR_AND:
            instrAnd();
            break;
          case INSTR_OR:
            instrOr();
            break;
          case INSTR_XOR:
            instrXor();
            break;
          case INSTR_NOT:
            instrNot();
            break;
          case INSTR_POP:
            instrPop();
            break;
          default:
            new_output_ = old_output_;
            Comm::Serial::send_error(Messages::UNKNOWN_INSTRUCTION);
            return false;
        }
      } else if (is_second_byte(byte)) {
        if (!is_first_byte(prevByte)) {
          new_output_ = old_output_;
          Comm::Serial::send_error(Messages::PREVIOUS_BYTE_ERROR);
          return false;
        }
        const uint8_t value = byte & DUAL_BYTE_MASK;
        const uint8_t instr = prevByte & DUAL_BYTE_MASK;
        if ((instr & INSTR_SET_MASK) == INSTR_SET) {
          instrSet(
            value,
            (instr & SET_VALUE_MASK) != 0 ? Value::High : Value::Low
          );
        } else if ((instr & INSTR_TOGGLE_MASK) == INSTR_TOGGLE) {
          instrToggle(
            value
          );
        } else if ((instr & INSTR_ON_MASK) == INSTR_ON) {
          instrOn(
            (instr & ON_EDGE_MASK) != 0 ? Edge::Rising : Edge::Falling,
            value
          );
        } else if ((instr & INSTR_IF_MASK) == INSTR_IF) {
          instrIf(
            (instr & IF_IO_MASK) != 0 ? InOut::Output : InOut::Input,
            (instr & IF_IS_WAS_MASK) != 0 ? IsWas::Is : IsWas::Was,
            value,
            (instr & IF_VALUE_MASK) != 0 ? Value::High : Value::Low
          );
        } else {
          new_output_ = old_output_;
          Comm::Serial::send_error(Messages::UNKNOWN_INSTRUCTION);
          return false;
        }
      } else if (!is_first_byte(byte)) {
        new_output_ = old_output_;
        Comm::Serial::send_error(Messages::UNKNOWN_INSTRUCTION);
        return false;
      }
      prevByte = byte;
    }
    new_output_ = old_output_;
    Comm::Serial::send_error(Messages::END_OF_PROGRAM);
    return false;
  }

  void VmContext::instrAnd() noexcept
  {
    auto b1 = stack_.pop();
    auto b2 = stack_.pop();
    stack_.push(b1 && b2);
  }

  void VmContext::instrOr() noexcept
  {
    auto b1 = stack_.pop();
    auto b2 = stack_.pop();
    stack_.push(b1 || b2);
  }

  void VmContext::instrXor() noexcept
  {
    auto b1 = stack_.pop();
    auto b2 = stack_.pop();
    stack_.push(b1 != b2);
  }

  void VmContext::instrNot() noexcept
  {
    auto b = stack_.pop();
    stack_.push(!b);
  }

  void VmContext::instrPop() noexcept
  {
    [[maybe_unused]] auto b = stack_.pop();
  }

  void VmContext::instrSet(uint8_t output, Value value) noexcept
  {
    if (!stack_.all_one()) {
      return;
    }
    new_output_.set(output, value == Value::High);
  }

  void VmContext::instrToggle(uint8_t output) noexcept
  {
    if (!stack_.all_one()) {
      return;
    }
    auto old = new_output_.get(output);
    new_output_.set(output, !old);
  }

  void VmContext::instrOn(Edge edge, uint8_t input) noexcept
  {
    switch (edge) {
      case Edge::Falling: {
        auto b = old_input_.get(input) && !new_input_.get(input);
        stack_.push(b);
        break;
      }
      case Edge::Rising: {
        auto b = !old_input_.get(input) && new_input_.get(input);
        stack_.push(b);
        break;
      }
      default:
        __builtin_unreachable();
    }
  }

  void VmContext::instrIf(InOut inOut, IsWas isWas, uint8_t n, Value value) noexcept
  {
    const Collections::BitSet32* set = nullptr;
    if (inOut == InOut::Input) {
      if (isWas == IsWas::Was) {
        set = &old_input_;
      } else {
        // isWas == IsWas::Is
        set = &new_input_;
      }
    } else {
      // inOut == InOut::Output
      if (isWas == IsWas::Was) {
        set = &old_output_;
      } else {
        // isWas == IsWas::Is
        set = &new_output_;
      }
    }
    if (value == Value::Low) {
      auto b = !set->get(n);
      stack_.push(b);
    } else {
      // value == Value::High
      auto b = set->get(n);
      stack_.push(b);
    }
  }

  Program::Program()
  {
    clear();
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

    uint16_t crc = calcCrc(code_, header_.length());

    return header_.crc() == crc;
  }

  void Program::clear()
  {
    using namespace Bytecode;
    code_[0] = INSTR_END;
    header_ = ProgramHeader(1, calcCrc(code_, 1));
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

  uint16_t Program::calcCrc(const uint8_t *buffer, uint16_t length) noexcept
  {
    if (length > sizeof(code_)) {
      return UINT16_MAX;
    }
    uint16_t crc = 0;
    for (auto i = UINT16_C(0); i < length; ++i) {
      crc = _crc_xmodem_update(crc, buffer[i]);
    }
    return crc;
  }

}
