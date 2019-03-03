#pragma once

#include <Arduino.h>

namespace StandaertHA {

size_t slip_encode(const byte * const in_buf,
                   const size_t in_size,
                   byte * const out_buf,
                   const size_t out_size);

}