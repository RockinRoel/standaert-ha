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

#include "constants.hpp"
#include "messages.hpp"
#include "state.hpp"

#include "collections/bitset32.hpp"
#include "comm/command.hpp"
#include "comm/message.hpp"
#include "comm/serial.hpp"
#include "hal/io.hpp"
#include "hal/mode.hpp"
#include "shal/interpreter.hpp"
#include "util/inet.hpp"
#include "util/slip.hpp"

#include <Arduino.h>
#include <Wire.h>

namespace StandaertHA {
  State state;

  [[nodiscard]] bool recv_message(State& state) noexcept;
  void handle_message(State& state) noexcept;
  void handle_command_message(State& state) noexcept;
  void handle_program_message(State& state) noexcept;
  void receive_program_data(State& state) noexcept;
  void abort_upload(State& state) noexcept;
  void finalize_program_upload(State& state) noexcept;
  void update_inputs(State& state) noexcept;
  bool run_program(State& state) noexcept;
  void update_outputs(const State& state, const Collections::BitSet32& output_before) noexcept;
  void send_update(State& state, const Collections::BitSet32& output_before) noexcept;

  // Update inputs
  void update_inputs(State& state) noexcept
  {
    HAL::IO::update_inputs(state);
  }

  // Receive message
  bool recv_message(State& state) noexcept
  {
    return Comm::Serial::receive(state);
  }

  void handle_command_message(State& state) noexcept {
    const auto& commandMsg = state.message.body_as_command_msg();
    for (uint8_t i = 0; i < state.message.body_length(); ++i) {
      const Comm::Command& command = commandMsg.command[i];
      if (command.type() == Comm::Command::Type::Refresh) {
        state.refresh = true;
      } else {
        state.output = command.apply(state.output);
      }
    }
  }

  void handle_program_message(State& state) noexcept {
    switch (state.message.type()) {
      case Comm::MessageType::ProgramStart: {
        state.upload_state.uploading = true;
        state.upload_state.position = 0;
        memcpy(&state.program.header(), &state.message.body_as_program_start().header, sizeof(Shal::Interpreter::ProgramHeader));
        Comm::Serial::send_program_start_ack(state.program.header());
        break;
      }
      case Comm::MessageType::ProgramEnd: {
        if (state.upload_state.uploading) {
          receive_program_data(state);
          finalize_program_upload(state);
        } else {
          Comm::Serial::send_error(Messages::UNEXPECTED_PROGRAM_END_ERROR);
          Comm::Serial::send_program_end_ack(state.program.header());
        }
        break;
      }
      case Comm::MessageType::ProgramData: {
        if (state.upload_state.uploading) {
          receive_program_data(state);
        } else {
          Comm::Serial::send_error(Messages::UNEXPECTED_PROGRAM_DATA_ERROR);
        }
        break;
      }
      default: {}
    }
  }

  void receive_program_data(State& state) noexcept {
    uint32_t new_size = static_cast<uint32_t>(state.upload_state.position) + static_cast<uint32_t>(state.message.body_length());
    bool size_error = false;
    if (new_size > static_cast<uint32_t>(Shal::Interpreter::MAX_CODE_SIZE)) {
      Comm::Serial::send_error(Messages::MAXIMUM_CODE_SIZE_ERROR);
      size_error = true;
    }
    if (!size_error && new_size > static_cast<uint32_t>(state.program.header().length())) {
      Comm::Serial::send_error(Messages::CODE_SIZE_MISMATCH_ERROR);
      size_error = true;
    }
    if (size_error) {
      abort_upload(state);
    }
    memcpy(state.program.code() + state.upload_state.position,
           state.message.body_as_program_data().code,
           state.message.body_length());
    state.upload_state.position += state.message.body_length();
  }

  void abort_upload(State& state) noexcept {
    state.upload_state.uploading = false;
    state.upload_state.position = 0;
    // reload program from EEPROM
    state.program.load();
  }

  void finalize_program_upload(State& state) noexcept {
    uint16_t program_size = state.upload_state.position;
    state.upload_state.uploading = false;
    state.upload_state.position = 0;
    if (program_size != static_cast<uint32_t>(state.program.header().length())) {
      Comm::Serial::send_error(Messages::CODE_SIZE_MISMATCH_ERROR);
      state.program.load();
      Comm::Serial::send_program_end_ack(state.program.header());
      return;
    }
    if (!state.program.verify()) {
      // Error: reload from EEPROM
      Comm::Serial::send_error(Messages::PROGRAM_VERIFICATION_ERROR);
      state.program.load();
      Comm::Serial::send_program_end_ack(state.program.header());
      return;
    }
    // Upload done, save to EEPROM
    state.program.save();
    Comm::Serial::send_program_end_ack(state.program.header());
  }

  // Handle messages
  void handle_message(State& state) noexcept {
    switch (state.message.type()) {
      case Comm::MessageType::Command:
        handle_command_message(state);
        break;
      case Comm::MessageType::ProgramStart:
      case Comm::MessageType::ProgramData:
      case Comm::MessageType::ProgramEnd:
        handle_program_message(state);
        break;
      default: {
        // Do nothing
      }
    }
  }

  // Run program
  bool run_program(State& state) noexcept {
    if (HAL::read_mode() == Mode::PROGRAM_DISABLED) {
      // Program is disabled
      return true;
    }
    if (state.upload_state.uploading) {
      // New program is being loaded
      return true;
    }
    const Collections::BitSet32 input_old(state.input.previous);
    const Collections::BitSet32 input_new(state.input.current);
    const Collections::BitSet32 output_old(state.output);
    Shal::Interpreter::VmContext vmContext(input_old, input_new, output_old);
    bool success = vmContext.run(state.program);
    state.output = vmContext.new_output();
    return success;
  }

  void update_outputs(const State& state, const Collections::BitSet32& output_before) noexcept {
    if (state.output != output_before) {
      HAL::IO::write_outputs(state.output);
    }
  }

  // Send messages
  void send_update(State& state, const Collections::BitSet32& output_before) noexcept {
    const bool refresh_requested = state.refresh;
    const bool input_changed = state.input.current != state.input.previous;
    const bool output_changed = state.output != output_before;
    if (refresh_requested || input_changed || output_changed) {
      Comm::Serial::send_update(state);
    }
    state.refresh = false;
  }
}

void setup() {
  using namespace StandaertHA;

  // Configure serial connection
  Serial.begin(Constants::SERIAL_BAUD_RATE);

  // Configure pins
  pinMode(Constants::MODE_PIN, INPUT);
  pinMode(Constants::MSB_PIN, INPUT);
  pinMode(LED_BUILTIN, OUTPUT);
  pinMode(Constants::RST_IN1_PIN, OUTPUT);
  pinMode(Constants::RST_IN2_PIN, OUTPUT);
  pinMode(Constants::RST_OUT1_PIN, OUTPUT);
  pinMode(Constants::RST_OUT2_PIN, OUTPUT);

  // Reset IO expanders
  HAL::IO::toggle_resets();

  // Init I2C bus
  Wire.begin();

  // Configure IO expanders
  HAL::IO::configure_inputs();
  HAL::IO::configure_outputs();

  // Setup debouncer state
  const Collections::BitSet32 inputs = HAL::IO::read_inputs();
  state.input.current = inputs;
  state.input.last_read = inputs;
  unsigned long t = millis();
  for (unsigned long& timestamp : state.input.timestamps) {
    timestamp = t;
  }

  // Load program
  state.program.load();
}

/**
 * Loop:
 *  - read inputs, get events
 *  - if MODE is 1:
 *    - process events
 *    - do postprocessing (e.g. ORring certain outputs)
 *  - transmit events and current output state over serial
 *  - receive and process commands over serial, until none are left
 *  - if MODE is 1:
 *    - do postprocessing (e.g. ORring certain outputs)
 */
void loop() {
  using namespace StandaertHA;

  const Collections::BitSet32 output_before = state.output;

  if (recv_message(state)) {
    handle_message(state);
  }
  update_inputs(state);
  bool success = run_program(state);
  digitalWrite(LED_BUILTIN, success ? LOW : HIGH);
  update_outputs(state, output_before);
  send_update(state, output_before);
}
