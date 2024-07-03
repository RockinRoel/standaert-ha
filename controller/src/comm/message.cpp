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

#include "comm/message.hpp"

#include "util/inet.hpp"

#include <util/crc16.h>

namespace StandaertHA::Comm {

Message::Message() noexcept
  : body_{
      .raw = {0},
    },
    type_(MessageType::Uninit),
    crc_(0),
    body_length_(0)
{}

uint16_t Message::calc_crc() const noexcept
{
  return calc_crc(type_, body_.raw, body_length_);
}

uint16_t Message::calc_crc(MessageType type, const uint8_t* body, uint8_t body_length) noexcept
{
  uint16_t crc = 0;
  crc = _crc_xmodem_update(crc, static_cast<uint8_t>(type));
  for (uint8_t i = 0; i < body_length; ++i) {
    // Note: strictly speaking this is UB, but it works
    crc = _crc_xmodem_update(crc, body[i]);
  }
  return crc;
}

Message::Message(const UpdateMsg& update, uint8_t event_count) noexcept
  : body_{
      .update = update,
    },
    type_(MessageType::Update)
{
  body_length_ = event_count + sizeof(update.outputs);
  crc_ = calc_crc();
}

Message::Message(const CommandMsg& command, uint8_t command_count) noexcept
  : body_{
      .command = command,
    },
    type_(MessageType::Command)
{
  body_length_ = command_count;
  crc_ = calc_crc();
}

Message::Message(const FailMsg& fail_msg, uint8_t size) noexcept
  : body_{
      .fail_msg = fail_msg,
    },
    type_(MessageType::Fail)
{
  body_length_ = size;
  crc_ = calc_crc();
}

Message::Message(const ProgramStart& program_start) noexcept
  : body_{
      .program_start = program_start,
    },
    type_(MessageType::ProgramStart)
{
  body_length_ = sizeof(ProgramStart);
  static_assert(sizeof(ProgramStart) == 8);
  crc_ = calc_crc();
}

Message::Message(const ProgramStartAck& program_start_ack) noexcept
  : body_{
      .program_start_ack = program_start_ack,
    },
    type_(MessageType::ProgramStartAck)
{
  body_length_ = sizeof(ProgramStartAck);
  static_assert(sizeof(ProgramStartAck) == 8);
  crc_ = calc_crc();
}

Message::Message(const ProgramData& program_data, uint8_t byte_count) noexcept
  : body_{
      .program_data = program_data,
    },
    type_(MessageType::ProgramData)
{
  body_length_ = byte_count;
  crc_ = calc_crc();
}

Message::Message(const ProgramEnd& program_end, uint8_t byte_count) noexcept
  : body_{
      .program_end = program_end,
    },
    type_(MessageType::ProgramEnd)
{
  body_length_ = byte_count;
  crc_ = calc_crc();
}

Message::Message(const ProgramEndAck& program_end_ack) noexcept
  : body_{
      .program_end_ack = program_end_ack,
    },
    type_(MessageType::ProgramEndAck)
{
  body_length_ = sizeof(ProgramEndAck);
  static_assert(sizeof(ProgramEndAck) == 8);
  crc_ = calc_crc();
}

Message Message::from_buffer(const uint8_t *buffer, uint8_t size) noexcept
{
  Message result;

  if (size < MIN_MESSAGE_LENGTH) {
    // Message too small, invalid
    return result;
  }

  if (size > MAX_MESSAGE_LENGTH) {
    // Message too large, invalid
    return result;
  }

  const uint16_t readCrc = (static_cast<uint16_t>(buffer[0]) << 8) |
                            static_cast<uint16_t>(buffer[1]);

  const uint8_t type = buffer[2];

  if (type == static_cast<uint8_t>(MessageType::ProgramStart) ||
      type == static_cast<uint8_t>(MessageType::ProgramStartAck) ||
      type == static_cast<uint8_t>(MessageType::ProgramEndAck)) {
    if (size - MESSAGE_HEADER_LENGTH != 8) {
      // Length is not exactly 8 bytes?
      return result;
    }
  } else if (type != static_cast<uint8_t>(MessageType::Update) &&
             type != static_cast<uint8_t>(MessageType::Command) &&
             type != static_cast<uint8_t>(MessageType::ProgramData) &&
             type != static_cast<uint8_t>(MessageType::ProgramEnd)) {
    // Unknown message
    return result;
  }

  const uint16_t calculatedCrc = calc_crc(static_cast<MessageType>(type), buffer + MESSAGE_HEADER_LENGTH,
                                          size - MESSAGE_HEADER_LENGTH);

  if (readCrc != calculatedCrc) {
    // CRC fail
    return result;
  }

  // Populate fields
  result.type_ = static_cast<MessageType>(type);
  result.crc_ = calculatedCrc;
  result.body_length_ = size - MESSAGE_HEADER_LENGTH;
  memcpy(result.body_.raw, buffer + MESSAGE_HEADER_LENGTH, size - MESSAGE_HEADER_LENGTH);

  return result;
}

uint8_t Message::to_buffer(uint8_t *buffer, uint8_t size) const noexcept {
  const uint8_t msgSize = MESSAGE_HEADER_LENGTH + body_length_;

  if (size < msgSize) {
    // Buffer too small
    return 0;
  }

  buffer[0] = static_cast<uint8_t>((crc_ >> 8) & 0xFF);
  buffer[1] = static_cast<uint8_t>(crc_ & 0xFF);
  buffer[2] = static_cast<uint8_t>(type_);

  memcpy(buffer + MESSAGE_HEADER_LENGTH, body_.raw, body_length_);

  return msgSize;
}

} // StandaertHA::Comm
