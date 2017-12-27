#include "I2CDevice.hpp"

#include "I2CBus.hpp"

namespace StandaertHA {
  namespace Hardware {

I2CDevice::I2CDevice(std::shared_ptr<I2CBus> bus, std::uint8_t chipAddress)
  : bus_{std::move(bus)}, chipAddress_{chipAddress}
{ }

std::vector<std::uint8_t> I2CDevice::read(std::uint8_t dataAddress, std::size_t count)
{
  return bus_->read(chipAddress_, dataAddress, count);
}

void I2CDevice::write(std::uint8_t dataAddress, std::vector<std::uint8_t> data)
{
  bus_->write(chipAddress_, dataAddress, data);
}

  }
}
