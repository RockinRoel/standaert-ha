#include "I2CBusDevice.hpp"

#include "I2CBus.hpp"

namespace StandaertHA {
  namespace Hardware {

I2CBusDevice::I2CBusDevice(
    std::shared_ptr<I2CBus> bus,
    std::uint8_t deviceAddess)
  : bus_{std::move(bus)},
    deviceAddress_{deviceAddess}
{ }

std::vector<std::uint8_t> I2CBusDevice::read(
    std::uint8_t dataAddress,
    std::size_t count)
{
  return bus_->read(deviceAddress_, dataAddress, count);
}

void I2CBusDevice::write(
    std::uint8_t dataAddress,
    const std::vector<std::uint8_t> &data)
{
  bus_->write(deviceAddress_, dataAddress, data);
}

  }
}
