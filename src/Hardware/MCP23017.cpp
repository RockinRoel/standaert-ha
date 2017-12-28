#include "MCP23017.hpp"

#include "I2CDevice.hpp"

namespace StandaertHA {
  namespace Hardware {

MCP23017::MCP23017(std::shared_ptr<I2CDevice> device)
  : device_{std::move(device)}
{ }

std::uint8_t MCP23017::read(register_t reg)
{
  return device_->read(static_cast<std::uint8_t>(reg), 1)[0];
}

void MCP23017::write(register_t reg, std::uint8_t value)
{
  device_->write(static_cast<std::uint8_t>(reg), {value});
}

  }
}
