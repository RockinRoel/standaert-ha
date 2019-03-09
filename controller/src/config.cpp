#include "config.h"

// Configuration tests

namespace StandaertHA {

namespace {

inline BUGGED_CONSTEXPR Config createConfig()
{
  const auto PressStart = ButtonEvent::Type::PressStart;
  const auto Toggle = Command::Type::Toggle;

  Config result;
  result.on(PressStart, 0, Toggle, 31);
  result.on(PressStart, 1, Toggle, 30);
  result.on(PressStart, 2, Toggle, 29);
  result.on(PressStart, 3, Toggle, 28);
  result.on(PressStart, 4, Toggle, 27);
  result.on(PressStart, 5, Toggle, 26);
  result.on(PressStart, 6, Toggle, 25);
  result.on(PressStart, 7, Toggle, 24);
  result.on(PressStart, 8, Toggle, 23);
  result.on(PressStart, 9, Toggle, 22);
  result.on(PressStart, 10, Toggle, 21);
  result.on(PressStart, 11, Toggle, 20);
  result.on(PressStart, 12, Toggle, 19);
  result.on(PressStart, 13, Toggle, 18);
  result.on(PressStart, 14, Toggle, 17);
  result.on(PressStart, 15, Toggle, 16);
  result.on(PressStart, 16, Toggle, 15);
  result.on(PressStart, 17, Toggle, 14);
  result.on(PressStart, 18, Toggle, 13);
  result.on(PressStart, 19, Toggle, 12);
  result.on(PressStart, 20, Toggle, 11);
  result.on(PressStart, 21, Toggle, 10);
  result.on(PressStart, 22, Toggle, 9);
  result.on(PressStart, 23, Toggle, 8);
  result.on(PressStart, 24, Toggle, 7);
  result.on(PressStart, 25, Toggle, 6);
  result.on(PressStart, 26, Toggle, 5);
  result.on(PressStart, 27, Toggle, 4);
  result.on(PressStart, 28, Toggle, 3);
  result.on(PressStart, 29, Toggle, 2);
  result.on(PressStart, 30, Toggle, 1);
  result.on(PressStart, 31, Toggle, 0);
  return result;
}

}

const Config config = createConfig();

void postprocess(uint32_t &output) {
  if (getBit(output, 0) == HIGH ||
      getBit(output, 1) == HIGH) {
    setBit(output, 2, HIGH);
  } else {
    setBit(output, 2, LOW);
  }
}

#ifndef BUGGED_COMPILER

namespace {

constexpr const Config testConfig =
  Config()
    .on(ButtonEvent::Type::PressStart, 0, Command::Type::Toggle, 31)
    .on(ButtonEvent::Type::PressStart, 1, Command::Type::Toggle, 30)
    .on(ButtonEvent::Type::PressStart, 2, Command::Type::Toggle, 29)
    .on(ButtonEvent::Type::PressStart, 3, Command::Type::Toggle, 28)
    .on(ButtonEvent::Type::PressStart, 4, Command::Type::Toggle, 27)
    .on(ButtonEvent::Type::PressStart, 5, Command::Type::Toggle, 26)
    .on(ButtonEvent::Type::PressStart, 6, Command::Type::Toggle, 25)
    .on(ButtonEvent::Type::PressStart, 7, Command::Type::Toggle, 24)
    .on(ButtonEvent::Type::PressStart, 8, Command::Type::Toggle, 23)
    .on(ButtonEvent::Type::PressStart, 9, Command::Type::Toggle, 22)
    .on(ButtonEvent::Type::PressStart, 10, Command::Type::Toggle, 21)
    .on(ButtonEvent::Type::PressStart, 11, Command::Type::Toggle, 20)
    .on(ButtonEvent::Type::PressStart, 12, Command::Type::Toggle, 19)
    .on(ButtonEvent::Type::PressStart, 13, Command::Type::Toggle, 18)
    .on(ButtonEvent::Type::PressStart, 14, Command::Type::Toggle, 17)
    .on(ButtonEvent::Type::PressStart, 15, Command::Type::Toggle, 16)
    .on(ButtonEvent::Type::PressStart, 16, Command::Type::Toggle, 15)
    .on(ButtonEvent::Type::PressStart, 17, Command::Type::Toggle, 14)
    .on(ButtonEvent::Type::PressStart, 18, Command::Type::Toggle, 13)
    .on(ButtonEvent::Type::PressStart, 19, Command::Type::Toggle, 12)
    .on(ButtonEvent::Type::PressStart, 20, Command::Type::Toggle, 11)
    .on(ButtonEvent::Type::PressStart, 21, Command::Type::Toggle, 10)
    .on(ButtonEvent::Type::PressStart, 22, Command::Type::Toggle, 9)
    .on(ButtonEvent::Type::PressStart, 23, Command::Type::Toggle, 8)
    .on(ButtonEvent::Type::PressStart, 24, Command::Type::Toggle, 7)
    .on(ButtonEvent::Type::PressStart, 25, Command::Type::Toggle, 6)
    .on(ButtonEvent::Type::PressStart, 26, Command::Type::Toggle, 5)
    .on(ButtonEvent::Type::PressStart, 27, Command::Type::Toggle, 4)
    .on(ButtonEvent::Type::PressStart, 28, Command::Type::Toggle, 3)
    .on(ButtonEvent::Type::PressStart, 29, Command::Type::Toggle, 2)
    .on(ButtonEvent::Type::PressStart, 30, Command::Type::Toggle, 1)
    .on(ButtonEvent::Type::PressStart, 31, Command::Type::Toggle, 0);

static_assert(testConfig.apply(0x00000000, ButtonEvent(0, ButtonEvent::Type::PressStart)) == 0x80000000, "toggle first light on");
static_assert(testConfig.apply(0x80000000, ButtonEvent(0, ButtonEvent::Type::PressStart)) == 0x00000000, "toggle first light off");
static_assert(testConfig.apply(0x00000000, ButtonEvent(31, ButtonEvent::Type::PressStart)) == 0x00000001, "toggle last light on");
static_assert(testConfig.apply(0x00000001, ButtonEvent(31, ButtonEvent::Type::PressStart)) == 0x00000000, "toggle last light off");

} // anonymous namespace

#endif // BUGGED_COMPILER

} // StandaertHA