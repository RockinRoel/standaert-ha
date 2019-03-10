#pragma once

#if STANDAERTHA_NATIVE
#include <cstdint>
#else // !STANDAERTHA_NATIVE
#include <Arduino.h>
#endif // !STANDAERTHA_NATIVE

#include "button_event.h"
#include "command.h"

#if defined(__GNUC__) && __GNUC__ < 6
#define BUGGED_COMPILER
#endif

#ifdef BUGGED_COMPILER
#define BUGGED_CONSTEXPR
#else // !BUGGED_COMPILER
#define BUGGED_CONSTEXPR constexpr
#endif // !BUGGED_COMPILER

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

extern const Config config;
extern void postprocess(uint32_t &output);

} // StandaertHA