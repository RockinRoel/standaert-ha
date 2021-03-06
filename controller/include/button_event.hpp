#pragma once

#include <Arduino.h>

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
  enum class Type : uint8_t {
    PressStart = 0x00,
    PressEnd   = 0x80
  };

  constexpr ButtonEvent()
    : data_(0)
  { }

  constexpr explicit ButtonEvent(uint8_t button, Type type)
    : data_(static_cast<uint8_t>(type) | // type
            (button & 0x1F) | // button id
            0x40) // valid
  { }
  
  constexpr bool valid() const {
    return data_ & 0x40;
  }

  constexpr Type type() const {
    return static_cast<Type>(data_ & 0x80);
  }

  constexpr uint8_t button() const {
    return data_ & 0x1F;
  }

  constexpr uint8_t raw() const {
    return data_;
  }

  static constexpr ButtonEvent fromRaw(uint8_t data) {
    return ButtonEvent(data);
  }

  constexpr bool operator==(const ButtonEvent &other) const {
    return data_ == other.data_;
  }

  constexpr bool operator!=(const ButtonEvent &other) const {
    return !operator==(other);
  }

private:
  constexpr explicit ButtonEvent(uint8_t data)
    : data_(data)
  { }

  uint8_t data_;
};

} // StandaertHA
