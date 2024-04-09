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

#include "util/slip.hpp"

namespace StandaertHA::Comm::Serial {

  void send(const Message& message) noexcept
  {
    byte buffer[Comm::MAX_MESSAGE_LENGTH];
    byte size = message.to_buffer(buffer, sizeof(buffer));
    if (size == 0) {
      // TODO(Roel): failure?
      return;
    }

    byte encoded_buf[sizeof(buffer) * 2 + 2];
    size_t encoded_size = Util::SLIP::encode(buffer, size, encoded_buf, sizeof(encoded_buf));

    ::Serial.write(encoded_buf, encoded_size);
  }

  void send_update(const State& state) noexcept
  {
    Comm::UpdateMsg update_msg;
    update_msg.outputs = Util::Inet::htonl(state.output.value());
    uint8_t event_count = 0;
    for (uint8_t i = 0; i < 32; ++i) {
      // TODO(Roel): bounds check? Should be safe...
      ButtonEvent& next_event = update_msg.events[event_count];
      const bool press_start = !state.input.current.get(i) && state.input.previous.get(i);
      const bool press_end = state.input.current.get(i) && !state.input.previous.get(i);
      if (press_start || press_end) {
        if (press_start) {
          next_event = ButtonEvent(i, ButtonEvent::Type::PressStart);
        } else {
          next_event = ButtonEvent(i, ButtonEvent::Type::PressEnd);
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
    Comm::FailMsg fail_msg;
    size_t i = 0;
    for (; i < size && i < sizeof(fail_msg.message); ++i) {
      fail_msg.message[i] = error_message[i];
    }

    Comm::Message message(fail_msg, i);
    send(message);
  }

}
