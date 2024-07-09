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

#include "comm/message.hpp"
#include "comm/serial.hpp"
#include "hal/io.hpp"
#include "shal/interpreter.hpp"

namespace StandaertHA {

  /**
   * Contains the state
   */
  struct State {
    /**
     * Output state, as an array of 32 bits, 1 for HIGH, 0 for LOW
     */
    Collections::BitSet32 output{0};

    /**
     * Serial state (input buffer)
     */
    struct Serial {
      size_t input_pos = 0;
      uint8_t input_buffer[Comm::MAX_MESSAGE_LENGTH * 2 + 2]; // message is max 128 bytes, times 2 to account for escapes, SLIP_END twice is 2 bytes
      Comm::Serial::RxState rx_state = Comm::Serial::RxState::SCAN;
    } serial;

    static_assert(sizeof(Serial::input_buffer) <= SIZE_MAX);

    /**
     * Debounced inputs
     */
    struct DebouncedInput {
      /**
       * Current committed state (after debouncing)
       *
       * Array of 32 bits, 1 for HIGH, 0 for LOW
       *
       * Inputs are pulled low when button press starts,
       * so 0 means the button is pressed, 1 means the
       * button is not pressed.
       */
      Collections::BitSet32 current{UINT32_MAX};

      /**
       * Previous committed state (after debouncing)
       *
       * Array of 32 bits, 1 for HIGH, 0 for LOW
       *
       * Inputs are pulled low when button press starts,
       * so 0 means the button is pressed, 1 means the
       * button is not pressed.
       */
      Collections::BitSet32 previous{UINT32_MAX};

      /**
       * Last measured state
       *
       * Array of 32 bits, 1 for HIGH, 0 for LOW
       */
      Collections::BitSet32 last_read{UINT32_MAX};

      /**
       * Timestamp when the last_read state was first measured
       */
      unsigned long timestamps[HAL::IO::NB_INPUTS];
    } input;

    /**
     * Need to send full state on next loop (refresh)
     */
    bool refresh = true;

    struct UploadState {
      bool uploading = false;
      uint16_t position = 0;
    } upload_state;

    Comm::Message message;

    Shal::Interpreter::Program program;

    void handle_message() noexcept;
    [[nodiscard]] bool run_program() noexcept;
    void update_outputs(const Collections::BitSet32& output_before) const noexcept;
    void send_update(const Collections::BitSet32& output_before) noexcept;

  private:
    void handle_command_message() noexcept;
    void handle_program_message() noexcept;
    void receive_program_data() noexcept;
    void abort_upload() noexcept;
    void finalize_program_upload() noexcept;
  };

}
