#include "mcp23017.hpp"

#include <iostream>
#include <string>

#include <linux/i2c.h>
#include <linux/i2c-dev.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/file.h>
#include <unistd.h>

static constexpr int address = 0x20;
static constexpr int commBuffSize = 1024;

enum class MCP23017_IOCON0_REGISTER : char {
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
  IOCONA = 0x0A,
  IOCONB = 0x0B, // Same contents as IOCONA
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

static constexpr const char* registerToString(MCP23017_IOCON0_REGISTER reg)
{
  switch (reg) {
    case MCP23017_IOCON0_REGISTER::IODIRA:
      return "IODIRA";
    case MCP23017_IOCON0_REGISTER::IODIRB:
      return "IODIRB";
    case MCP23017_IOCON0_REGISTER::IPOLA:
      return "IPOLA";
    case MCP23017_IOCON0_REGISTER::IPOLB:
      return "IPOLB";
    case MCP23017_IOCON0_REGISTER::GPINTENA:
      return "GPINTENA";
    case MCP23017_IOCON0_REGISTER::GPINTENB:
      return "GPINTENB";
    case MCP23017_IOCON0_REGISTER::DEFVALA:
      return "DEFVALA";
    case MCP23017_IOCON0_REGISTER::DEFVALB:
      return "DEFVALB";
    case MCP23017_IOCON0_REGISTER::INTCONA:
      return "INTCONA";
    case MCP23017_IOCON0_REGISTER::INTCONB:
      return "INTCONB";
    case MCP23017_IOCON0_REGISTER::IOCONA:
      return "IOCON";
    case MCP23017_IOCON0_REGISTER::IOCONB:
      return "IOCON (copy)";
    case MCP23017_IOCON0_REGISTER::GPPUA:
      return "GPPUA";
    case MCP23017_IOCON0_REGISTER::GPPUB:
      return "GPPUB";
    case MCP23017_IOCON0_REGISTER::INTFA:
      return "INTFA";
    case MCP23017_IOCON0_REGISTER::INTFB:
      return "INTFB";
    case MCP23017_IOCON0_REGISTER::INTCAPA:
      return "INTCAPA";
    case MCP23017_IOCON0_REGISTER::INTCAPB:
      return "INTCAPB";
    case MCP23017_IOCON0_REGISTER::GPIOA:
      return "GPIOA";
    case MCP23017_IOCON0_REGISTER::GPIOB:
      return "GPIOB";
    case MCP23017_IOCON0_REGISTER::OLATA:
      return "OLATA";
    case MCP23017_IOCON0_REGISTER::OLATB:
      return "OLATB";
    default:
      return "Unknown";
  }
}

int test_i2c()
{
  int fd = open("/dev/i2c-0", O_RDWR);
  if (fd < 0) {
    std::cout << "ERROR OPENING I2C DEV" << std::endl;
    return 1;
  }

  if (flock(fd, LOCK_EX) < 0) {
    std::cout << "ERROR PLACING EXCLUSIVE LOCK ON I2C DEV" << std::endl;
    return 1;
  }

  if (ioctl(fd, I2C_SLAVE, address) < 0) {
    std::cout << "ERROR SETTING I2C ADDRESS" << std::endl;
    return 1;
  }

  char buf[commBuffSize];
  buf[0] = 0x00;
  if (write(fd, buf, 1) != 1) {
    std::cout << "ERROR WRITING 0x00" << std::endl;
  }

  if (read(fd, buf, 0x16) != 0x16) {
    std::cout << "ERROR READING REGISTERS" << std::endl;
  } else {
    std::cout << "REGISTER CONTENTS:" << std::endl;
    for (int i = 0; i < 0x16; ++i) {
      std::cout << "  " << registerToString(static_cast<MCP23017_IOCON0_REGISTER>(i)) << " " << (int)buf[i] << std::endl;
    }
  }

  buf[0] = 0x0A;
  buf[1] = 0x40; // Set mirror bit
  if (write(fd, buf, 2) != 2) {
    std::cout << "Error writing IOCON register" << std::endl;
  }
}
