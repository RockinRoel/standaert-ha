#pragma once

#include "button_event.hpp"

namespace StandaertHA::DSL {

class Event;

class Context {
private:
  const ButtonEvent &currentEvent_;
  const uint32_t outputBefore_;
  uint32_t outputAfter_;

public:
  constexpr Context(const ButtonEvent &currentEvent,
                    uint32_t outputBefore);

  constexpr uint32_t outputAfter() const { return outputAfter_; }

  inline constexpr Event on(ButtonEvent onEvent);
  inline constexpr Event onPressStart(uint8_t button);
  inline constexpr Event onPressEnd(uint8_t button);

private:
  friend class Event;
};

class Event {
private:
  Context &ctx_;
  ButtonEvent onEvent_;

public:
  constexpr inline Event toggle(uint8_t output) &&;
  constexpr inline Event turnOn(uint8_t output) &&;
  constexpr inline Event switchOff(uint8_t output) &&;
  
private:
  constexpr Event(Context &ctx,
                  ButtonEvent onEvent);

  friend class Context;
};

}

#include "dsl.ipp"
