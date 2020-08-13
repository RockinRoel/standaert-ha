#pragma once

constexpr inline int getBit(const uint32_t i, const uint8_t bit) {
  return ((i >> bit) & 1) ? HIGH : LOW;
}

constexpr inline void setBit(uint32_t &i, const uint8_t bit, int v) {
  uint32_t mask = ((uint32_t)1) << bit;
  if (v == HIGH) {
    i = i | mask;
  } else {
    i = i & ~mask;
  }
}