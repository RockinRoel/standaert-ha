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

#include "comm/message.hpp"
#include "constants.hpp"
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
      uint8_t input_pos = 0;
      uint8_t input_buffer[Comm::MAX_MESSAGE_LENGTH * 2 + 2]; // message is max 128 bytes, times 2 to account for escapes, SLIP_END twice is 2 bytes
    } serial;

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
      Collections::BitSet32 current{0xFFFFFFFF};

      /**
       * Previous committed state (after debouncing)
       *
       * Array of 32 bits, 1 for HIGH, 0 for LOW
       *
       * Inputs are pulled low when button press starts,
       * so 0 means the button is pressed, 1 means the
       * button is not pressed.
       */
      Collections::BitSet32 previous{0xFFFFFFFF};

      /**
       * Last measured state
       *
       * Array of 32 bits, 1 for HIGH, 0 for LOW
       */
      Collections::BitSet32 last_read{0xFFFFFFFF};

      /**
       * Timestamp when the last_read state was first measured
       */
      unsigned long timestamps[32];
    } input;

    /**
     * Need to send full state on next loop (refresh)
     */
    bool refresh = true;

    struct UploadState {
      bool uploading = false;
      uint16_t position = 0;
    } upload_state;

    struct Error {
      const char* message = nullptr;
      size_t size = 0;

      template<size_t size>
      void set_error(const char (&error_message)[size])
      {
        message = error_message;
        this->size = size - 1;
      }

      void reset_error()
      {
        message = nullptr;
        size = 0;
      }
    } error;

    Comm::Message message;

    Shal::Interpreter::Program program;
  };

}
