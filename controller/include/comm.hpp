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

#include "button_event.hpp"
#include "command.hpp"

#include "shal/interpreter.hpp"

#include <Arduino.h>

namespace StandaertHA::Comm {

constexpr size_t MESSAGE_HEADER_LENGTH = 3;
constexpr size_t MIN_MESSAGE_LENGTH = MESSAGE_HEADER_LENGTH;
constexpr size_t MAX_MESSAGE_LENGTH = 128;
constexpr size_t MAX_MESSAGE_BODY_LENGTH = MAX_MESSAGE_LENGTH - MESSAGE_HEADER_LENGTH;

enum class MsgType : uint8_t {
  Unknown = 0x00,

  Update = 'u', // Update from controller (max. 32 events (32 bytes) + output state (4 bytes))
  Command = 'c', // Commands from host

  ProgramStart = 's', // Start transmit program (program header (8 bytes))
  ProgramStartAck = 'S', // Acknowledge transmit program (program header (8 bytes))
  ProgramData = 'd', // Send program data (middle) (127 bytes)
  ProgramEnd = 'e', // End of program data (max. 127 bytes)
  ProgramEndAck = 'E', // Acknowledge program end (program header (8 bytes))
};

struct UpdateMsg {
  uint32_t outputs;
  ButtonEvent events[MAX_MESSAGE_BODY_LENGTH - sizeof(outputs)];
} __attribute__((packed));

struct CommandMsg {
  Command command[MAX_MESSAGE_BODY_LENGTH];
} __attribute__((packed));

struct ProgramStart {
  Shal::Interpreter::ProgramHeader header;
} __attribute__((packed));

struct ProgramStartAck {
  Shal::Interpreter::ProgramHeader header;
} __attribute__((packed));

struct ProgramData {
  uint8_t code[MAX_MESSAGE_BODY_LENGTH];
} __attribute__((packed));

struct ProgramEnd {
  uint8_t code[MAX_MESSAGE_BODY_LENGTH];
} __attribute__((packed));

struct ProgramEndAck {
  Shal::Interpreter::ProgramHeader header;
} __attribute__((packed));

union MsgBody {
  UpdateMsg update;
  CommandMsg command;
  ProgramStart programStart;
  ProgramStartAck programStartAck;
  ProgramData programData;
  ProgramEnd programEnd;
  ProgramEndAck programEndAck;
  uint8_t raw[MAX_MESSAGE_BODY_LENGTH];
} __attribute__((packed));

static_assert(sizeof(MsgBody) == MAX_MESSAGE_BODY_LENGTH);

// On the wire:
// CRC (2 bytes) + type (1 byte) + body (at most MAX_MESSAGE_BODY_LENGTH bytes)
class Message {
public:
  Message();

  explicit Message(const UpdateMsg& update, uint8_t nbEvents);
  explicit Message(const CommandMsg& command, uint8_t nbCommands);
  explicit Message(const ProgramStart& programStart);
  explicit Message(const ProgramStartAck& ProgramStartAck);
  explicit Message(const ProgramData& programData, uint8_t nbBytes);
  explicit Message(const ProgramEnd& programEnd, uint8_t nbBytes);
  explicit Message(const ProgramEndAck& ProgramEndAck);

  MsgType type() const { return type_; }
  uint8_t bodyLength() const { return bodyLength_; }

  const UpdateMsg& bodyAsUpdate() const { return body_.update; }
  const CommandMsg& bodyAsCommandMsg() const { return body_.command; }
  const ProgramStart& bodyAsProgramStart() const { return body_.programStart; }
  const ProgramStartAck& bodyAsProgramStartAck() const { return body_.programStartAck; }
  const ProgramData& bodyAsProgramData() const { return body_.programData; }
  const ProgramEnd& bodyAsProgramEnd() const { return body_.programEnd; }
  const ProgramEndAck& bodyAsProgramEndAck() const { return body_.programEndAck; }

  static Message fromBuffer(const uint8_t* buffer, uint8_t size);
  // Returns written amount of data
  uint8_t toBuffer(uint8_t* buffer, uint8_t size) const;

private:
  MsgBody body_;
  MsgType type_;
  uint16_t crc_ = 0;
  uint8_t bodyLength_ = 0;

  [[nodiscard]] uint16_t calcCrc() const;
  [[nodiscard]] static uint16_t calcCrc(MsgType type, const uint8_t* body, uint8_t bodyLength);
};

}