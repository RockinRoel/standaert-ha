#include "comm.hpp"

#include "command.hpp"
#include "inet.hpp"

#include <util/crc16.h>

namespace StandaertHA::Comm {


Message::Message()
  : body_{
      .raw = {0},
    },
    type_(MsgType::Unknown),
    crc_(0),
    bodyLength_(0)
{}

uint16_t Message::calcCrc() const
{
  return calcCrc(type_, body_.raw, bodyLength_);
}

uint16_t Message::calcCrc(MsgType type, const uint8_t* body, uint8_t bodyLength)
{
  uint16_t crc = 0;
  crc = _crc_xmodem_update(crc, static_cast<uint8_t>(type));
  for (uint8_t i = 0; i < bodyLength; ++i) {
    // Note: strictly speaking this is UB, but it works
    crc = _crc_xmodem_update(crc, body[i]);
  }
  return crc;
}

Message::Message(const UpdateMsg& update, uint8_t nbEvents)
  : body_{
      .update = update,
    },
    type_(MsgType::Update)
{
  bodyLength_ = nbEvents + sizeof(update.outputs);
  crc_ = calcCrc();
}

Message::Message(const CommandMsg& command, uint8_t nbCommands)
  : body_{
      .command = command,
    },
    type_(MsgType::Command)
{
  bodyLength_ = nbCommands;
  crc_ = calcCrc();
}

Message::Message(const ProgramStart& programStart)
  : body_{
      .programStart = programStart,
    },
    type_(MsgType::ProgramStart)
{
  bodyLength_ = sizeof(ProgramStart);
  static_assert(sizeof(ProgramStart) == 8);
  crc_ = calcCrc();
}

Message::Message(const ProgramStartAck& programStartAck)
  : body_{
      .programStartAck = programStartAck,
    },
    type_(MsgType::ProgramStartAck)
{
  bodyLength_ = sizeof(ProgramStartAck);
  static_assert(sizeof(ProgramStartAck) == 8);
  crc_ = calcCrc();
}

Message::Message(const ProgramData& programData, uint8_t nbBytes)
  : body_{
      .programData = programData,
    },
    type_(MsgType::ProgramData)
{
  bodyLength_ = nbBytes;
  crc_ = calcCrc();
}

Message::Message(const ProgramEnd& programEnd, uint8_t nbBytes)
  : body_{
      .programEnd = programEnd,
    },
    type_(MsgType::ProgramEnd)
{
  bodyLength_ = nbBytes;
  crc_ = calcCrc();
}

Message::Message(const ProgramEndAck& programEndAck)
  : body_{
      .programEndAck = programEndAck,
    },
    type_(MsgType::ProgramEndAck)
{
  bodyLength_ = sizeof(ProgramEndAck);
  static_assert(sizeof(ProgramEndAck) == 8);
  crc_ = calcCrc();
}

Message Message::fromBuffer(const uint8_t * const buffer, const uint8_t size)
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

  if (type == static_cast<uint8_t>(MsgType::ProgramStart) ||
      type == static_cast<uint8_t>(MsgType::ProgramStartAck) ||
      type == static_cast<uint8_t>(MsgType::ProgramEndAck)) {
    if (size - MESSAGE_HEADER_LENGTH != 8) {
      // Length is not exactly 8 bytes?
      return result;
    }
  } else if (type != static_cast<uint8_t>(MsgType::Update) &&
             type != static_cast<uint8_t>(MsgType::Command) &&
             type != static_cast<uint8_t>(MsgType::ProgramData) &&
             type != static_cast<uint8_t>(MsgType::ProgramEnd)) {
    // Unknown message
    return result;
  }

  const uint16_t calculatedCrc = calcCrc(static_cast<MsgType>(type), buffer + MESSAGE_HEADER_LENGTH, size - MESSAGE_HEADER_LENGTH);

  if (readCrc != calculatedCrc) {
    // CRC fail
    return result;
  }

  // Populate fields
  result.type_ = static_cast<MsgType>(type);
  result.crc_ = calculatedCrc;
  result.bodyLength_ = size - MESSAGE_HEADER_LENGTH;
  memcpy(result.body_.raw, buffer + MESSAGE_HEADER_LENGTH, size - MESSAGE_HEADER_LENGTH);

  return result;
}

uint8_t Message::toBuffer(uint8_t* const buffer, const uint8_t size) const {
  const uint8_t msgSize = MESSAGE_HEADER_LENGTH + bodyLength_;

  if (size < msgSize) {
    // Buffer too small
    return 0;
  }

  buffer[0] = static_cast<uint8_t>((crc_ >> 8) & 0xFF);
  buffer[1] = static_cast<uint8_t>(crc_ & 0xFF);
  buffer[2] = static_cast<uint8_t>(type_);

  memcpy(buffer + MESSAGE_HEADER_LENGTH, body_.raw, bodyLength_);

  return msgSize;
}

}