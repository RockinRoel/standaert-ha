#pragma once

#include "I2CDevice.hpp"

#include <memory>

namespace StandaertHA {
  namespace Hardware {

class I2CBus;

class I2CBusDevice final : public I2CDevice {
public:
  I2CBusDevice(std::shared_ptr<I2CBus> bus, std::uint8_t deviceAddress);

  virtual std::vector<std::uint8_t> read(std::uint8_t dataAddress, std::size_t count) override;
  virtual void write(std::uint8_t dataAddress, const std::vector<std::uint8_t> &data) override;

private:
  std::shared_ptr<I2CBus> bus_;
  std::uint8_t deviceAddress_;
};

  }
}
