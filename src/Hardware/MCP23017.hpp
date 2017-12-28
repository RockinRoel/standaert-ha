#pragma once

#include <cstdint>
#include <memory>

namespace StandaertHA {
  namespace Hardware {

class I2CDevice;

class MCP23017 final {
public:
  enum class register_t : std::uint8_t {
    IODIRA = 0x00,
    IODIRB = 0x01,
    IPOLA = 0x02,
    IPOLB = 0x03,
    GPINTENA = 0x04,
    GPINTENB = 0x05,
    DEFVALA = 0x06,
    DEFVALB = 0x07,
    INTCONA = 0x08,
    INTCONB = 0x09,
    IOCON = 0x0A,
    IOCON_ALT = 0x0B,
    GPPUA = 0x0C,
    GPPUB = 0x0D,
    INTFA = 0x0E,
    INTFB = 0x0F,
    INTCAPA = 0x10,
    INTCAPB = 0x11,
    GPIOA = 0x12,
    GPIOB = 0x13,
    OLATA = 0x14,
    OLATB = 0x15
  };

  MCP23017(std::shared_ptr<I2CDevice> device);

  std::uint8_t read(register_t reg);
  void write(register_t reg, std::uint8_t value);

private:
  std::shared_ptr<I2CDevice> device_;
};

  }
}
