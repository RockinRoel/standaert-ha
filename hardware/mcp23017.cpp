#include "mcp23017.hpp"

#include <iostream>

#include <linux/i2c.h>
#include <linux/i2c-dev.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <unistd.h>

static constexpr int address = 0x20;
static constexpr int commBuffSize = 1024;

int test_i2c()
{
  int fd = open("/dev/i2c-0", O_RDWR);
  if (fd < 0) {
    std::cout << "ERROR OPENING I2C DEV" << std::endl;
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

  if (read(fd, buf, 1) != 1) {
    std::cout << "ERROR READING" << std::endl;
  } else {
    std::cout << "READ DATA: " << (int)buf[0] << std::endl;
  }

  buf[0] = 0x01;
  if (write(fd, buf, 1) != 1) {
    std::cout << "ERROR WRITING 0x01" << std::endl;
  }

  if (read(fd, buf, 1) != 1) {
    std::cout << "ERROR READING" << std::endl;
  } else {
    std::cout << "READ DATA: " << (int)buf[0] << std::endl;
  }

  buf[0] = 0x02;
  if (write(fd, buf, 1) != 1) {
    std::cout << "ERROR WRITING 0x02" << std::endl;
  }

  if (read(fd, buf, 1) != 1) {
    std::cout << "ERROR READING" << std::endl;
  } else {
    std::cout << "READ DATA: " << (int)buf[0] << std::endl;
  }
}
