#pragma once

#include <cstdint>
#include <memory>
#include <vector>

namespace StandaertHA {
  namespace Hardware {

class I2CBus;

class I2CDevice {
public:
  I2CDevice(std::shared_ptr<I2CBus> bus, std::uint8_t chipAddress);

  std::vector<std::uint8_t> read(std::uint8_t dataAddress, std::size_t count = 1);
  void write(std::uint8_t dataAddress, std::vector<std::uint8_t> data);

private:
  std::shared_ptr<I2CBus> bus_;
  std::uint8_t chipAddress_;
};

  }
}
