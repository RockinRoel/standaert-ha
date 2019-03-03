#pragma once

#include <Arduino.h>

namespace StandaertHA {

/**
 * Command, encoded as:
 * 
 * TT0IIIII
 * 
 * TT: command type
 * IIIII: output id
 */
class Command {
public:
  enum class Type : uint8_t {
    None   = 0x00,
    Toggle = 0x40,
    Off    = 0x80,
    On     = 0xC0
  };
  
  constexpr Command()
    : data_(0)
  { }

  constexpr explicit Command(Type type, uint8_t output)
    : data_(static_cast<uint8_t>(type) | // type
            (output & 0x1F)) // output
  { }

  constexpr Command(const Command &) = default;
  Command &operator=(const Command &) = default;

  Command(Command &&c)
    : data_(c.data_)
  {
    c.data_ = 0;
  }

  Command &operator=(Command &&c)
  {
    data_ = c.data_;
    if (this != &c)
      c.data_ = 0;
    
    return *this;
  }

  constexpr Type type() const {
    return static_cast<Type>(data_ & 0xC0);
  }

  constexpr uint8_t output() const {
    return data_ & 0x1F;
  }

  constexpr uint8_t raw() const {
    return data_;
  }

  constexpr static Command fromRaw(uint8_t data) {
    return Command(data);
  }

private:
  constexpr explicit Command(uint8_t data)
    : data_(data)
  { }

  uint8_t data_;
};

} // StandaertHA