#pragma once

#include "Arduino.h"

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
  enum class Type : byte {
    None   = B00000000,
    Toggle = B01000000,
    Off    = B10000000,
    On     = B11000000
  };
  
  constexpr Command()
    : data_(0)
  { }

  constexpr explicit Command(Type type, byte output)
    : data_(static_cast<byte>(type) | // type
            (output & B00011111)) // output
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
    return static_cast<Type>(data_ & B11000000);
  }

  constexpr byte output() const {
    return data_ & B00011111;
  }

  constexpr byte raw() const {
    return data_;
  }

  constexpr static Command fromRaw(byte data) {
    return Command(data);
  }

private:
  constexpr explicit Command(byte data)
    : data_(data)
  { }

  byte data_;
};

} // StandaertHA