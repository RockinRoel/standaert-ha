// Copyright (C) Roel Standaert
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#pragma once

// We don't actually have cstdint and cstring headers
// NOLINTBEGIN(modernize-deprecated-headers)
#include <stdint.h>
#include <string.h>
// NOLINTEND(modernize-deprecated-headers)

namespace StandaertHA::Util::Inet {

  /**
   * \brief Host to network order (for 32-bit unsigned integers)
   *
   * Converts an unsigned 32-bit integer from host order
   * (usually little-endian) to network order (big-endian).
   */
  inline uint32_t htonl(uint32_t hostlong) noexcept {
    uint8_t buffer[4];
    buffer[0] = (hostlong >> 24) & 0xFF;
    buffer[1] = (hostlong >> 16) & 0xFF;
    buffer[2] = (hostlong >> 8) & 0xFF;
    buffer[3] = hostlong & 0xFF;
    uint32_t result;
    memcpy(&result, buffer, 4);
    return result;
  }

  /**
   * \brief Network to host order (for 32-bit unsigned integers)
   *
   * Converts an unsigned 32-bit integer from network order
   * (big-endian) to network order (usually little-endian).
   */
  inline uint32_t ntohl(uint32_t networklong) noexcept {
    uint8_t buffer[4];
    memcpy(buffer, &networklong, 4);
    return (static_cast<uint32_t>(buffer[0]) << 24) |
           (static_cast<uint32_t>(buffer[1]) << 16) |
           (static_cast<uint32_t>(buffer[2]) << 8) |
           static_cast<uint32_t>(buffer[3]);
  }

  /**
   * \brief Host to network order (for 16-bit unsigned integers)
   *
   * Converts an unsigned 16-bit integer from host order
   * (usually little-endian) to network order (big-endian).
   */
  inline uint16_t htons(uint16_t hostshort) noexcept {
    uint8_t buffer[2];
    buffer[0] = (hostshort >> 8) & 0xFF;
    buffer[1] = hostshort & 0xFF;
    uint16_t result;
    memcpy(&result, buffer, 2);
    return result;
  }

  /**
   * \brief Network to host order (for 16-bit unsigned integers)
   *
   * Converts an unsigned 16-bit integer from network order
   * (big-endian) to network order (usually little-endian).
   */
  inline uint16_t ntohs(uint16_t networkshort) noexcept {
    uint8_t buffer[2];
    memcpy(buffer, &networkshort, 2);
    return (static_cast<uint16_t>(buffer[0]) << 8) |
           static_cast<uint16_t>(buffer[1]);
  }

} // StandaertHA::Util
