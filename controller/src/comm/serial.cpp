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

#include "comm/serial.hpp"

#include "state.hpp"
#include "hal/io.hpp"
#include "util/slip.hpp"

#include <Arduino.h>

namespace StandaertHA::Comm::Serial {

  bool receive(State& state) noexcept
  {
    // Reset message
    state.message = Comm::Message();
    while (::Serial.available() > 0) {
      int b = ::Serial.read();
      if (state.serial.rx_state == RxState::SCAN) {
        if (b == Util::SLIP::END) {
          state.serial.input_buffer[0] = Util::SLIP::END;
          state.serial.input_pos = 1;
          state.serial.rx_state = RxState::READ;
        }
      } else {
        if (state.serial.input_pos == sizeof(state.serial.input_buffer) - 1) {
          // Buffer full, should not happen, discard buffer
          state.serial.rx_state = RxState::SCAN;
          state.serial.input_pos = 0;
          continue;
        }
        state.serial.input_buffer[state.serial.input_pos++] = b;
        if (b == Util::SLIP::END) {
          if (state.serial.input_pos >= 5) { // SLIP_END (1) + DATA (MIN 1) + CRC (2) + SLIP_END (1)
            uint8_t decoded_buf[Comm::MAX_MESSAGE_LENGTH];
            size_t decoded_size = 0;
            bool success = Util::SLIP::decode(state.serial.input_buffer,
                                              state.serial.input_pos,
                                              decoded_buf,
                                              sizeof(decoded_buf),
                                              decoded_size);
            if (success) {
              state.message = Comm::Message::from_buffer(decoded_buf, decoded_size);
            }
            state.serial.rx_state = RxState::SCAN;
            state.serial.input_pos = 0;
            return success;
          } else {
            // discard!
          }
          state.serial.rx_state = RxState::SCAN;
          state.serial.input_pos = 0;
          return false;
        }
      }
    }
    return false;
  }

  void send(const Message& message) noexcept
  {
    uint8_t buffer[Comm::MAX_MESSAGE_LENGTH];
    uint8_t size = message.to_buffer(buffer, sizeof(buffer));
    if (size == 0) {
      return;
    }

    uint8_t encoded_buf[sizeof(buffer) * 2 + 2];
    size_t encoded_size = 0;
    bool success = Util::SLIP::encode(buffer, size, encoded_buf, sizeof(encoded_buf), encoded_size);
    if (!success) {
      return;
    }

    ::Serial.write(encoded_buf, encoded_size);
  }

  void send_update(const State& state) noexcept
  {
    Comm::UpdateMsg update_msg;
    update_msg.outputs = Util::Inet::htonl(state.output.value());
    uint8_t event_count = 0;
    for (uint8_t i = 0; i < HAL::IO::NB_INPUTS; ++i) {
      // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
      Event& next_event = update_msg.events[event_count];
      const bool fedge = !state.input.current.get(i) && state.input.previous.get(i);
      const bool redge = state.input.current.get(i) && !state.input.previous.get(i);
      if (redge || fedge) {
        if (redge) {
          next_event = Event(Event::Type::RisingEdge, i);
        } else {
          next_event = Event(Event::Type::FallingEdge, i);
        }
        ++event_count;
      }
    }

    Comm::Message message(update_msg, event_count);
    send(message);
  }

  void send_program_start_ack(const Shal::Interpreter::ProgramHeader& header) noexcept
  {
    Comm::ProgramStartAck program_start_ack;
    program_start_ack.header = header;

    Comm::Message message(program_start_ack);
    send(message);
  }

  void send_program_end_ack(const Shal::Interpreter::ProgramHeader& header) noexcept
  {
    Comm::ProgramEndAck program_end_ack;
    program_end_ack.header = header;

    Comm::Message message(program_end_ack);
    send(message);
  }

  void send_error(const char * const error_message, const size_t size) noexcept
  {
    Comm::FailMsg fail_msg{};
    size_t i = 0;
    for (; i < size && i < sizeof(fail_msg.message); ++i) {
      // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
      fail_msg.message[i] = pgm_read_byte_near(error_message + i);
    }

    Comm::Message message(fail_msg, i);
    send(message);
  }

  void send_info(const char * const info_message, const size_t size) noexcept
  {
    Comm::InfoMsg info_msg{};
    size_t i = 0;
    for (; i < size && i < sizeof(info_msg.message); ++i) {
      // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-constant-array-index)
      info_msg.message[i] = pgm_read_byte_near(info_message + i);
    }

    Comm::Message message(info_msg, i);
    send(message);
  }

}
