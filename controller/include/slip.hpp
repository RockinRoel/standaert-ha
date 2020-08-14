#pragma once

#include <Arduino.h>

namespace StandaertHA {

constexpr const uint8_t SLIP_END = 0xC0;

size_t slip_encode(const uint8_t * const in_buf,
                   const size_t in_size,
                   uint8_t * const out_buf,
                   const size_t out_size);

size_t slip_decode(const uint8_t * const in_buf,
                   const size_t in_size,
                   uint8_t * const out_buf,
                   const size_t out_size);

}
