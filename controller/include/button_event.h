#pragma once

#include "Arduino.h"

namespace StandaertHA {

/**
 * ButtonEvent, encoded as:
 * 
 * TV0IIIII
 * ^^     ^
 * ||     \- Button id
 * |\------- Valid
 * \-------- Type
 * 
 * e.g.
 * 01000010 : second button press start
 * 11000010 : second button press end
 * 00000000 : not valid
 */
class ButtonEvent {
public:
  enum class Type : byte {
    PressStart = B00000000,
    PressEnd   = B10000000
  };

  constexpr ButtonEvent()
    : data_(0)
  { }

  constexpr explicit ButtonEvent(byte button,
                        Type type)
    : data_(static_cast<byte>(type) | // type
            (button & B00011111) | // button id
            B01000000) // valid
  { }

  constexpr ButtonEvent(const ButtonEvent &e) = default;
  ButtonEvent &operator=(const ButtonEvent &e) = default;

  ButtonEvent(ButtonEvent &&e)
    : data_(e.data_)
  {
    e.data_ = 0;
  }

  ButtonEvent &operator=(ButtonEvent &&e)
  {
    data_ = e.data_;
    if (this != &e)
      e.data_ = 0;
    
    return *this;
  }
  
  constexpr bool valid() const {
    return data_ & B01000000;
  }

  constexpr Type type() const {
    return static_cast<Type>(data_ & B10000000);
  }

  constexpr byte button() const {
    return data_ & B00011111;
  }

  constexpr byte raw() const {
    return data_;
  }

  static constexpr ButtonEvent fromRaw(byte data) {
    return ButtonEvent(data);
  }

private:
  constexpr explicit ButtonEvent(byte data)
    : data_(data)
  { }

  byte data_;
};

} // StandaertHA