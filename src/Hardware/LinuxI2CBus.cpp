#include "LinuxI2CBus.hpp"

// TODO(Roel): remove?
// #define DEBUG_LOG 1

#include <algorithm>
#include <array>
#include <iomanip>
#include <iostream>

extern "C" {
#include <linux/i2c-dev.h>
#include <sys/ioctl.h>
#include <sys/file.h>
#include <unistd.h>
}

namespace {
static constexpr ssize_t BUF_SIZE = 512;
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

#ifdef DEBUG_LOG
  std::cerr << "Opened I2C device: " << fd_ << "\n";
#endif // DEBUG_LOG
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
  ssize_t bytesRead = ::read(fd_, output.data(), scount);
  if (bytesRead != scount) {
    std::cerr << "Err\n"; // TODO(Roel)
  }

#ifdef DEBUG_LOG
  std::cerr << "Read " << bytesRead << " bytes from chip "
            << "0x" << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(chipAddress)
            << ", data address "
            << "0x" << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(dataAddress)
            << "\n";
  std::cerr << "DATA:";
  for (ssize_t i = 0; i < bytesRead; ++i) {
    std::cerr << " " << std::uppercase << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(output[i]);
  }
  std::cerr << "\n";
#endif // DEBUG_LOG

  return output;
}

void LinuxI2CBus::write(std::uint8_t chipAddress, std::uint8_t dataAddress, const std::vector<std::uint8_t> &data)
{
  if (ioctl(fd_, I2C_SLAVE, chipAddress) < 0) {
    std::cerr << "Err\n"; // TODO(Roel)
  }

  ssize_t ssize = data.size() + 1;
  if (ssize > BUF_SIZE) {
    std::cerr << "Err\n"; // TODO(Roel)
  }
  std::array<std::uint8_t, BUF_SIZE> buf;
  buf[0] = dataAddress;
  std::copy(data.begin(), data.end(), buf.begin() + 1);

  ssize_t bytesWrote = ::write(fd_, buf.data(), ssize);
  if (bytesWrote != ssize) {
    std::cerr << "Err\n"; // TODO(Roel)
  }

#ifdef DEBUG_LOG
  std::cerr << "Wrote " << bytesWrote << " bytes to chip "
            << "0x" << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(chipAddress)
            << ", data address "
            << "0x" << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(dataAddress)
            << "\n";
  std::cerr << "DATA:";
  for (ssize_t i = 0; i < bytesWrote; ++i) {
    std::cerr << " " << std::uppercase << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(buf[i]);
  }
  std::cerr << "\n";
#endif // DEBUG_LOG
}

  }
}
