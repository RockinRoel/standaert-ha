#pragma once

#include <Arduino.h>

#include "dsl.hpp"

namespace StandaertHA {

constexpr inline void handleEvent(DSL::Context &c)
{
  c.onPressStart(0).toggle(4);
  c.onPressStart(1).toggle(3);
  c.onPressStart(2).toggle(24);
  c.onPressStart(3).toggle(2);
  c.onPressStart(4).toggle(0);
  c.onPressStart(5).toggle(17);
  c.onPressStart(6).toggle(20);
  c.onPressStart(7).toggle(13);
  c.onPressStart(8).toggle(16);
  c.onPressStart(9).toggle(6);
  c.onPressStart(11)
    .toggle(25)
    .toggle(5);
  c.onPressStart(12).toggle(7);
  c.onPressStart(13)
    .toggle(5)
    .toggle(25);
  c.onPressStart(14).toggle(31);
  c.onPressStart(15).toggle(20);
  c.onPressStart(16)
    .toggle(5)
    .toggle(25);
  c.onPressStart(17)
    .toggle(28)
    .toggle(29);
  c.onPressStart(18).toggle(26);
  c.onPressStart(19).toggle(11);
  c.onPressStart(20).toggle(30);
  c.onPressStart(21).toggle(8);
  c.onPressStart(22).toggle(18);
  c.onPressStart(23).toggle(14);
  c.onPressStart(24).toggle(15);
  c.onPressStart(25).toggle(15);
  c.onPressStart(26).toggle(12);
  c.onPressStart(27).toggle(27);
  c.onPressStart(28).toggle(12);
  c.onPressStart(29).toggle(10);
  c.onPressStart(30).toggle(15);
  c.onPressStart(31).toggle(23);
}

constexpr inline void postprocess(uint32_t &output)
{
  if (getBit(output, 20) == HIGH ||
      getBit(output, 5) == HIGH) {
    setBit(output, 22, HIGH);
  } else {
    setBit(output, 22, LOW);
  }
}

} // StandaertHA
