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

#include "event.hpp"
#include "command.hpp"
#include "util/pred.hpp"

#include "shal/interpreter.hpp"

#include <Arduino.h>

namespace StandaertHA::Comm {

  constexpr size_t MESSAGE_HEADER_LENGTH = 3;
  constexpr size_t MIN_MESSAGE_LENGTH = MESSAGE_HEADER_LENGTH;
  constexpr size_t MAX_MESSAGE_LENGTH = 128;
  constexpr size_t MAX_MESSAGE_BODY_LENGTH = MAX_MESSAGE_LENGTH - MESSAGE_HEADER_LENGTH;

  enum class MessageType : uint8_t {
    Uninit = 0,

    Update = 'u', // Update from controller (output state + button events)
    Command = 'c', // Commands from host
    Fail = 'F', // Error message
    Info = 'I', // Info message

    ProgramStart = 's', // Start transmit program (program header (8 bytes))
    ProgramStartAck = 'S', // Acknowledge transmit program (program header (8 bytes))
    ProgramData = 'd', // Send program data (middle) (127 bytes)
    ProgramEnd = 'e', // End of program data (max. 127 bytes)
    ProgramEndAck = 'E', // Acknowledge program end (program header (8 bytes))
  };

  static_assert(
    Util::Pred::all_different(
      MessageType::Update,
      MessageType::Command,
      MessageType::Fail,
      MessageType::ProgramStart,
      MessageType::ProgramStartAck,
      MessageType::ProgramData,
      MessageType::ProgramEnd,
      MessageType::ProgramEndAck
    ),
    "All message types should be different!"
  );

  struct UpdateMsg {
    uint32_t outputs;
    Event events[MAX_MESSAGE_BODY_LENGTH - sizeof(outputs)];
  } __attribute__((packed));

  static_assert(sizeof(UpdateMsg) <= MAX_MESSAGE_BODY_LENGTH);

  struct CommandMsg {
    Command command[MAX_MESSAGE_BODY_LENGTH];
  } __attribute__((packed));

  static_assert(sizeof(CommandMsg) <= MAX_MESSAGE_BODY_LENGTH);

  struct FailMsg {
    unsigned char message[MAX_MESSAGE_BODY_LENGTH];
  } __attribute__((packed));

  static_assert(sizeof(FailMsg) <= MAX_MESSAGE_BODY_LENGTH);

  struct InfoMsg {
    unsigned char message[MAX_MESSAGE_BODY_LENGTH];
  } __attribute__((packed));

  static_assert(sizeof(InfoMsg) <= MAX_MESSAGE_BODY_LENGTH);

  struct ProgramStart {
    Shal::Interpreter::ProgramHeader header;
  } __attribute__((packed));

  static_assert(sizeof(ProgramStart) <= MAX_MESSAGE_BODY_LENGTH);

  struct ProgramStartAck {
    Shal::Interpreter::ProgramHeader header;
  } __attribute__((packed));

  static_assert(sizeof(ProgramStartAck) <= MAX_MESSAGE_BODY_LENGTH);

  struct ProgramData {
    uint8_t code[MAX_MESSAGE_BODY_LENGTH];
  } __attribute__((packed));

  static_assert(sizeof(ProgramData) <= MAX_MESSAGE_BODY_LENGTH);

  struct ProgramEnd {
    uint8_t code[MAX_MESSAGE_BODY_LENGTH];
  } __attribute__((packed));

  static_assert(sizeof(ProgramEnd) <= MAX_MESSAGE_BODY_LENGTH);

  struct ProgramEndAck {
    Shal::Interpreter::ProgramHeader header;
  } __attribute__((packed));

  static_assert(sizeof(ProgramEndAck) <= MAX_MESSAGE_BODY_LENGTH);

  union MsgBody {
    UpdateMsg update;
    CommandMsg command;
    FailMsg fail_msg;
    InfoMsg info_msg;
    ProgramStart program_start;
    ProgramStartAck program_start_ack;
    ProgramData program_data;
    ProgramEnd program_end;
    ProgramEndAck program_end_ack;
    uint8_t raw[MAX_MESSAGE_BODY_LENGTH];
  } __attribute__((packed));

  static_assert(sizeof(MsgBody) == MAX_MESSAGE_BODY_LENGTH);

  // Over the wire:
  // CRC (2 bytes) + type (1 byte) + body (at most MAX_MESSAGE_BODY_LENGTH bytes)
  class Message {
  public:
    Message() noexcept;

    explicit Message(const UpdateMsg& update, uint8_t event_count) noexcept;
    explicit Message(const CommandMsg& command, uint8_t command_count) noexcept;
    explicit Message(const FailMsg& fail_msg, uint8_t size) noexcept;
    explicit Message(const InfoMsg& info_msg, uint8_t size) noexcept;
    explicit Message(const ProgramStart& program_start) noexcept;
    explicit Message(const ProgramStartAck& program_start_ack) noexcept;
    explicit Message(const ProgramData& program_data, uint8_t byte_count) noexcept;
    explicit Message(const ProgramEnd& program_end, uint8_t byte_count) noexcept;
    explicit Message(const ProgramEndAck& program_end_ack) noexcept;

    [[nodiscard]] constexpr MessageType type() const noexcept { return type_; }
    [[nodiscard]] constexpr uint8_t msg_length() const noexcept { return body_length() + sizeof(crc_) + sizeof(type_); }
    [[nodiscard]] constexpr uint8_t body_length() const noexcept { return body_length_; }

    [[nodiscard]] constexpr const UpdateMsg& body_as_update() const noexcept { return body_.update; }
    [[nodiscard]] constexpr const CommandMsg& body_as_command_msg() const noexcept { return body_.command; }
    [[nodiscard]] constexpr const FailMsg& body_as_fail_msg() const noexcept { return body_.fail_msg; }
    [[nodiscard]] constexpr const InfoMsg& body_as_info_msg() const noexcept { return body_.info_msg; }
    [[nodiscard]] constexpr const ProgramStart& body_as_program_start() const noexcept { return body_.program_start; }
    [[nodiscard]] constexpr const ProgramStartAck& body_as_program_start_ack() const noexcept { return body_.program_start_ack; }
    [[nodiscard]] constexpr const ProgramData& body_as_program_data() const noexcept { return body_.program_data; }
    [[nodiscard]] constexpr const ProgramEnd& body_as_program_end() const noexcept { return body_.program_end; }
    [[nodiscard]] constexpr const ProgramEndAck& body_as_program_end_ack() const noexcept { return body_.program_end_ack; }

    [[nodiscard]] static Message from_buffer(const uint8_t *buffer, uint8_t size) noexcept;
    // Returns written amount of data
    [[nodiscard]] uint8_t to_buffer(uint8_t* buffer, uint8_t size) const noexcept;

  private:
    MsgBody body_;
    MessageType type_;
    uint16_t crc_ = 0;
    uint8_t body_length_ = 0;

    [[nodiscard]] uint16_t calc_crc() const noexcept;
    [[nodiscard]] static uint16_t calc_crc(MessageType type, const uint8_t* body, uint8_t body_length) noexcept;
  };

}
