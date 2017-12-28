#pragma once

#include <cstdint>
#include <vector>

namespace StandaertHA {
  namespace Hardware {

class I2CBus {
public:
  I2CBus();
  virtual ~I2CBus();

  virtual std::vector<std::uint8_t> read(std::uint8_t chipAddress, std::uint8_t dataAddress, std::size_t count) = 0;
  virtual void write(std::uint8_t chipAddress, std::uint8_t dataAddress, const std::vector<std::uint8_t> &data) = 0;
};

  }
}
