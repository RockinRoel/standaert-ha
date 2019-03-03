#include "slip.h"

namespace StandaertHA {

namespace {

constexpr const byte SLIP_END = 0xC0;
constexpr const byte SLIP_ESC = 0xDB;
constexpr const byte SLIP_ESC_END = 0xDC;
constexpr const byte SLIP_ESC_ESC = 0xDD;

}

size_t slip_encode(const byte * const in_buf,
                   const size_t in_size,
                   byte * const out_buf,
                   const size_t out_size)
{
  size_t pos = 0;
  out_buf[pos++] = SLIP_END;

  for (size_t in_pos = 0; in_pos < in_size; ++in_pos) {
    const byte b = in_buf[in_pos];
    if (b == SLIP_END) {
      out_buf[pos++] = SLIP_ESC;
      out_buf[pos++] = SLIP_ESC_END;
    } else if (b == SLIP_ESC) {
      out_buf[pos++] = SLIP_ESC;
      out_buf[pos++] = SLIP_ESC_ESC;
    } else {
      out_buf[pos++] = b;
    }
  }

  out_buf[pos++] = SLIP_END;
  return pos;
}

size_t slip_decode(const byte * const in_buf,
                   const size_t in_size,
                   byte * const out_buf,
                   const size_t out_size)
{
  size_t in_pos = 0;
  while (in_buf[in_pos] != SLIP_END) {
      ++in_pos;
  }
  ++in_pos;
  size_t pos = 0;
  while (in_pos < in_size &&
         in_buf[in_pos] != SLIP_END) {
    if (in_buf[in_pos] == SLIP_ESC) {
        ++in_pos;
        if (in_buf[in_pos] == SLIP_ESC_END) {
            out_buf[pos++] = SLIP_END;
        } else if (in_buf[in_pos] == SLIP_ESC_ESC) {
            out_buf[pos++] = SLIP_ESC;
        } else {
            // TODO: error?
        }
        ++in_pos;
    } else {
        out_buf[pos++] = in_buf[in_pos];
    }
  }
  return pos;
}

}