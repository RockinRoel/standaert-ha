#include "config.hpp"

// Configuration tests

namespace StandaertHA {

namespace {

constexpr void testHandleEvent(DSL::Context &c)
{
  c.onPressStart(0).toggle(31);
  c.onPressStart(1).toggle(30);
  c.onPressStart(2).toggle(29);
  c.onPressStart(3).toggle(28);
  c.onPressStart(4).toggle(27);
  c.onPressStart(5).toggle(26);
  c.onPressStart(6).toggle(25);
  c.onPressStart(7).toggle(24);
  c.onPressStart(8).toggle(23);
  c.onPressStart(9).toggle(22);
  c.onPressStart(10).toggle(21);
  c.onPressStart(11).toggle(20);
  c.onPressStart(12).toggle(19);
  c.onPressStart(13).toggle(18);
  c.onPressStart(14).toggle(17);
  c.onPressStart(15).toggle(16);
  c.onPressStart(16).toggle(15);
  c.onPressStart(17).toggle(14);
  c.onPressStart(18).toggle(13);
  c.onPressStart(19).toggle(12);
  c.onPressStart(20).toggle(11);
  c.onPressStart(21).toggle(10);
  c.onPressStart(22).toggle(9);
  c.onPressStart(23).toggle(8);
  c.onPressStart(24).toggle(7);
  c.onPressStart(25).toggle(6);
  c.onPressStart(26).toggle(5);
  c.onPressStart(27).toggle(4);
  c.onPressStart(28).toggle(3);
  c.onPressStart(29).toggle(2);
  c.onPressStart(30).toggle(1);
  c.onPressStart(31).toggle(0);
}

constexpr uint32_t testApplyEvent(const ButtonEvent &event, uint32_t stateBefore) {
  DSL::Context c(event, stateBefore);
  testHandleEvent(c);
  return c.outputAfter();
}

static_assert(testApplyEvent(ButtonEvent(0, ButtonEvent::Type::PressStart), 0x00000000) == 0x80000000, "toggle first light on");
static_assert(testApplyEvent(ButtonEvent(0, ButtonEvent::Type::PressStart), 0x80000000) == 0x00000000, "toggle first light off");
static_assert(testApplyEvent(ButtonEvent(31, ButtonEvent::Type::PressStart), 0x00000000) == 0x00000001, "toggle last light on");
static_assert(testApplyEvent(ButtonEvent(31, ButtonEvent::Type::PressStart), 0x00000001) == 0x00000000, "toggle last light off");

} // anonymous namespace

} // StandaertHA
