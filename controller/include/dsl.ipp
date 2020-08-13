#pragma once

#include "dsl.hpp"
#include "util.hpp"

namespace StandaertHA::DSL {

constexpr Context::Context(const ButtonEvent &currentEvent,
                           uint32_t outputBefore)
  : currentEvent_(currentEvent),
    outputBefore_(outputBefore),
    outputAfter_(outputBefore)
{ }

constexpr Event Context::on(ButtonEvent evt)
{
  return Event(*this, evt);
}

constexpr Event Context::onPressStart(uint8_t button)
{
  return Event(*this, ButtonEvent(button, ButtonEvent::Type::PressStart));
}

constexpr Event Context::onPressEnd(uint8_t button)
{
  return Event(*this, ButtonEvent(button, ButtonEvent::Type::PressEnd));
}

constexpr Event::Event(Context &ctx,
                       ButtonEvent onEvent)
  : ctx_(ctx),
    onEvent_(onEvent)
{ }

constexpr Event Event::toggle(uint8_t output) &&
{
  if (onEvent_ != ctx_.currentEvent_)
    return *this;
  
  const int before = getBit(ctx_.outputBefore_, output);
  if (before == HIGH) {
    setBit(ctx_.outputAfter_, output, LOW);
  } else {
    setBit(ctx_.outputAfter_, output, HIGH);
  }

  return *this;
}

constexpr Event Event::turnOn(uint8_t output) &&
{
  if (onEvent_ != ctx_.currentEvent_)
    return *this;

  setBit(ctx_.outputAfter_, output, HIGH);

  return *this;
}

constexpr Event Event::switchOff(uint8_t output) &&
{
  if (onEvent_ != ctx_.currentEvent_)
    return *this;

  setBit(ctx_.outputAfter_, output, LOW);

  return *this;
}

}
