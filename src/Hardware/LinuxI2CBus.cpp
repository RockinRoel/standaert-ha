#include "LinuxI2CBus.hpp"

#include <iostream>

extern "C" {
#include <linux/i2c-dev.h>
#include <sys/ioctl.h>
#include <sys/file.h>
#include <unistd.h>
}

namespace StandaertHA {
  namespace Hardware {

LinuxI2CBus::LinuxI2CBus(const std::string &pathToDev)
  : fd_{-1}
{
  fd_ = open(pathToDev.c_str(), O_RDWR|O_CLOEXEC|O_SYNC);
  if (fd_ < 0) {
    std::cerr << "Err\n"; // TODO(Roel)
  }

  if (flock(fd_, LOCK_EX) < 0) {
    std::cerr << "Err\n"; // TODO(Roel)
  }
}

LinuxI2CBus::~LinuxI2CBus()
{
  if (fd_ >= 0)
    close(fd_);
}

std::vector<std::uint8_t> LinuxI2CBus::read(std::uint8_t chipAddress, std::uint8_t dataAddress, std::size_t count)
{
  std::vector<std::uint8_t> output;
  output.resize(count);

  if (ioctl(fd_, I2C_SLAVE, chipAddress) < 0) {
    std::cerr << "Err\n"; // TODO(Roel)
  }

  if (::write(fd_, &dataAddress, 1) != 1) {
    std::cerr << "Err\n"; // TODO(Roel)
  }

  ssize_t scount = static_cast<ssize_t>(count);
  if (::read(fd_, output.data(), scount) != scount) {
    std::cerr << "Err\n"; // TODO(Roel)
  }

  return output;
}

void LinuxI2CBus::write(std::uint8_t chipAddress, std::uint8_t dataAddress, const std::vector<std::uint8_t> &data)
{
  if (ioctl(fd_, I2C_SLAVE, chipAddress) < 0) {
    std::cerr << "Err\n"; // TODO(Roel)
  }

  if (::write(fd_, &dataAddress, 1) != 1) {
    std::cerr << "Err\n"; // TODO(Roel)
  }

  ssize_t ssize = data.size();
  if (::write(fd_, data.data(), ssize) != ssize) {
    std::cerr << "Err\n"; // TODO(Roel)
  }
}

  }
}
