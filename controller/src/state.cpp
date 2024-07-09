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

#include "state.hpp"

#include "messages.hpp"
#include "hal/mode.hpp"

namespace StandaertHA {

  void State::handle_message() noexcept
  {
    switch (message.type()) {
      case Comm::MessageType::Command: {
        handle_command_message();
      }
        break;
      case Comm::MessageType::ProgramStart:
      case Comm::MessageType::ProgramData:
      case Comm::MessageType::ProgramEnd:
        handle_program_message();
        break;
      default: {
        // Do nothing
      }
    }
  }

  bool State::run_program() noexcept
  {
    if (HAL::read_mode() == Mode::PROGRAM_DISABLED) {
      // Program is disabled
      return true;
    }
    if (upload_state.uploading) {
      // New program is being loaded
      return true;
    }
    const Collections::BitSet32 input_old(input.previous);
    const Collections::BitSet32 input_new(input.current);
    const Collections::BitSet32 output_old(output);
    Shal::Interpreter::VmContext vmContext(input_old, input_new, output_old);
    bool success = vmContext.run(program);
    output = vmContext.new_output();
    return success;
  }

  void State::update_outputs(const Collections::BitSet32 &output_before) const noexcept
  {
    if (output != output_before) {
      HAL::IO::write_outputs(output);
    }
  }

  void State::send_update(const Collections::BitSet32 &output_before) noexcept
  {
    const bool refresh_requested = refresh;
    const bool input_changed = input.current != input.previous;
    const bool output_changed = output != output_before;
    if (refresh_requested || input_changed || output_changed) {
      Comm::Serial::send_update(*this);
    }
    refresh = false;
  }

  void State::handle_command_message() noexcept
  {
    const auto& commandMsg = message.body_as_command_msg();
    for (uint8_t i = 0; i < message.body_length(); ++i) {
      // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
      const Comm::Command& command = commandMsg.command[i];
      if (command.type() == Comm::Command::Type::Refresh) {
        refresh = true;
      } else {
        output = command.apply(output);
      }
    }
  }

  void State::handle_program_message() noexcept {
    switch (message.type()) {
      case Comm::MessageType::ProgramStart: {
        upload_state.uploading = true;
        upload_state.position = 0;
        memcpy(&program.header(), &message.body_as_program_start().header, sizeof(Shal::Interpreter::ProgramHeader));
        Comm::Serial::send_program_start_ack(program.header());
        break;
      }
      case Comm::MessageType::ProgramEnd: {
        if (upload_state.uploading) {
          receive_program_data();
          finalize_program_upload();
        } else {
          Comm::Serial::send_error(Messages::UNEXPECTED_PROGRAM_END_ERROR);
          Comm::Serial::send_program_end_ack(program.header());
        }
        break;
      }
      case Comm::MessageType::ProgramData: {
        if (upload_state.uploading) {
          receive_program_data();
        } else {
          Comm::Serial::send_error(Messages::UNEXPECTED_PROGRAM_DATA_ERROR);
        }
        break;
      }
      default: {}
    }
  }

  void State::receive_program_data() noexcept {
    uint32_t new_size = static_cast<uint32_t>(upload_state.position) + static_cast<uint32_t>(message.body_length());
    bool size_error = false;
    if (new_size > static_cast<uint32_t>(Shal::Interpreter::MAX_CODE_SIZE)) {
      Comm::Serial::send_error(Messages::MAXIMUM_CODE_SIZE_ERROR);
      size_error = true;
    }
    if (!size_error && new_size > static_cast<uint32_t>(program.header().length())) {
      Comm::Serial::send_error(Messages::CODE_SIZE_MISMATCH_ERROR);
      size_error = true;
    }
    if (size_error) {
      abort_upload();
    }
    memcpy(program.code() + upload_state.position,
           message.body_as_program_data().code,
           message.body_length());
    upload_state.position += message.body_length();
  }

  void State::abort_upload() noexcept {
    upload_state.uploading = false;
    upload_state.position = 0;
    // reload program from EEPROM
    program.load();
  }

  void State::finalize_program_upload() noexcept {
    uint16_t program_size = upload_state.position;
    upload_state.uploading = false;
    upload_state.position = 0;
    if (program_size != static_cast<uint32_t>(program.header().length())) {
      Comm::Serial::send_error(Messages::CODE_SIZE_MISMATCH_ERROR);
      program.load();
      Comm::Serial::send_program_end_ack(program.header());
      return;
    }
    if (!program.verify()) {
      // Error: reload from EEPROM
      Comm::Serial::send_error(Messages::PROGRAM_VERIFICATION_ERROR);
      program.load();
      Comm::Serial::send_program_end_ack(program.header());
      return;
    }
    // Upload done, save to EEPROM
    program.save();
    Comm::Serial::send_program_end_ack(program.header());
  }

}
