#pragma once

#if STANDAERTHA_NATIVE
#include <cstddef>
#include <cstdint>
#else // !STANDAERTHA_NATIVE
#include <Arduino.h>
#endif // !STANDAERTHA_NATIVE

namespace StandaertHA {

size_t slip_encode(const uint8_t * const in_buf,
                   const size_t in_size,
                   uint8_t * const out_buf,
                   const size_t out_size);

}