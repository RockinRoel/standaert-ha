#pragma once

#if STANDAERTHA_NATIVE
#include <cstdint>
#else // !STANDAERTHA_NATIVE
#include <Arduino.h>
#endif // !STANDAERTHA_NATIVE

#include "button_event.h"
#include "command.h"

namespace StandaertHA {

class Config {
public:
   constexpr Config()
     : pressStart_(CommandSet<32>()),
       pressEnd_(CommandSet<32>())
   { }

   constexpr Config &on(const ButtonEvent::Type eventType,
                        const uint8_t button,
                        const Command::Type commandType,
                        const uint8_t output)
   {
     if (eventType == ButtonEvent::Type::PressStart) {
       pressStart_.addCommand(button, commandType, output);
     } else if (eventType == ButtonEvent::Type::PressEnd) {
       pressEnd_.addCommand(button, commandType, output);
     }
     return *this;
   }

   constexpr uint32_t apply(const uint32_t before,
                            const ButtonEvent &event) const
   {
     if (!event.valid())
       return before;

     if (event.type() == ButtonEvent::Type::PressStart) {
       return pressStart_.apply(before, event.button());
     } else if (event.type() == ButtonEvent::Type::PressEnd) {
       return pressEnd_.apply(before, event.button());
     }

     return before;
   }

private:
  template<int NbIn>
  class CommandSet {
  public:
      constexpr CommandSet()
        : on_{},
          off_{},
          toggle_{}
      { }

      constexpr void addCommand(const uint8_t button,
                                const Command::Type commandType,
                                const uint8_t output)
      {
        if (commandType == Command::Type::On) {
          on_[button] = on_[button] | (static_cast<uint32_t>(1) << output);
        } else if (commandType == Command::Type::Off) {
          off_[button] = off_[button] | (static_cast<uint32_t>(1) << output);
        } else if (commandType == Command::Type::Toggle) {
          toggle_[button] = toggle_[button] | (static_cast<uint32_t>(1) << output);
        }
      }

      constexpr uint32_t apply(const uint32_t before,
                               const uint8_t button) const
      {
        uint32_t result = before;
        result = (result | on_[button]);
        result = (result & ~off_[button]);
        result = (result ^ toggle_[button]);
        return result;
      }

  private:
      uint32_t on_[NbIn];
      uint32_t off_[NbIn];
      uint32_t toggle_[NbIn];
  };

  constexpr explicit Config(const CommandSet<32> pressStart,
                            const CommandSet<32> pressEnd)
    : pressStart_(pressStart),
      pressEnd_(pressEnd)
  { }

  CommandSet<32> pressStart_;
  CommandSet<32> pressEnd_;
};

namespace Impl {

inline constexpr Config createConfig()
{
  const auto PressStart = ButtonEvent::Type::PressStart;
  const auto Toggle = Command::Type::Toggle;

  Config result;
  result.on(PressStart, 0, Toggle, 4);
  result.on(PressStart, 1, Toggle, 3);
  result.on(PressStart, 2, Toggle, 24);
  result.on(PressStart, 3, Toggle, 2);
  result.on(PressStart, 4, Toggle, 0);
  result.on(PressStart, 5, Toggle, 17);
  result.on(PressStart, 6, Toggle, 20);
  result.on(PressStart, 7, Toggle, 13);
  result.on(PressStart, 8, Toggle, 16);
  result.on(PressStart, 9, Toggle, 6);
  result.on(PressStart, 11, Toggle, 25);
  result.on(PressStart, 11, Toggle, 5);
  result.on(PressStart, 12, Toggle, 7);
  result.on(PressStart, 13, Toggle, 5);
  result.on(PressStart, 13, Toggle, 25);
  result.on(PressStart, 14, Toggle, 31);
  result.on(PressStart, 15, Toggle, 20);
  result.on(PressStart, 16, Toggle, 5);
  result.on(PressStart, 16, Toggle, 25);
  result.on(PressStart, 17, Toggle, 28);
  result.on(PressStart, 17, Toggle, 29);
  result.on(PressStart, 18, Toggle, 26);
  result.on(PressStart, 19, Toggle, 11);
  result.on(PressStart, 20, Toggle, 30);
  result.on(PressStart, 21, Toggle, 8);
  result.on(PressStart, 22, Toggle, 18);
  result.on(PressStart, 23, Toggle, 14);
  result.on(PressStart, 24, Toggle, 15);
  result.on(PressStart, 25, Toggle, 15);
  result.on(PressStart, 26, Toggle, 12);
  result.on(PressStart, 27, Toggle, 27);
  result.on(PressStart, 28, Toggle, 12);
  result.on(PressStart, 29, Toggle, 10);
  result.on(PressStart, 30, Toggle, 15);
  result.on(PressStart, 31, Toggle, 23);
  return result;
}

}

constexpr Config config = Impl::createConfig();

inline void postprocess(uint32_t &output)
{
  if (getBit(output, 20) == HIGH ||
      getBit(output, 5) == HIGH) {
    setBit(output, 22, HIGH);
  } else {
    setBit(output, 22, LOW);
  }
}

} // StandaertHA
