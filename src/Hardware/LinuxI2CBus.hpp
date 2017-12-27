#pragma once

#include "I2CBus.hpp"

#include <string>

namespace StandaertHA {
  namespace Hardware {

class LinuxI2CBus final : public I2CBus {
public:
  LinuxI2CBus(const std::string &pathToDev);
  ~LinuxI2CBus();

  virtual std::vector<std::uint8_t> read(std::uint8_t chipAddress, std::uint8_t dataAddress, std::size_t count) override;
  virtual void write(std::uint8_t chipAddress, std::uint8_t dataAddress, std::vector<std::uint8_t> data) override;

private:
  int fd_;
};

  }
}
