#define IN 0
#define OUT 1

#define LOW 0
#define HIGH 1

#define POUT 23

#include <unistd.h>

#include <fcntl.h>

#include <stdio.h>
#include <iostream>

#include <chrono>
#include <thread>

static int
GPIOExport(int pin)
{
#define BUFFER_MAX 3
  char buffer[BUFFER_MAX];
  ssize_t bytes_written;
  int fd;

  fd = open("/sys/class/gpio/export", O_WRONLY|O_CLOEXEC|O_SYNC);
  if (-1 == fd) {
    std::cerr << "Failed to open export for writing!" << std::endl;
    return -1;
  }

  bytes_written = snprintf(buffer, BUFFER_MAX, "%d", pin);
  write(fd, buffer, bytes_written);
  close(fd);
  return 0;
}

static int
GPIOUnexport(int pin)
{
  char buffer[BUFFER_MAX];
  ssize_t bytes_written;
  int fd;

  fd = open("/sys/class/gpio/unexport", O_WRONLY|O_CLOEXEC|O_SYNC);
  if (-1 == fd) {
    std::cerr << "Failed to open unexport for writing!" << std::endl;
    return -1;
  }

  bytes_written = snprintf(buffer, BUFFER_MAX, "%d", pin);
  write(fd, buffer, bytes_written);
  close(fd);
  return 0;
}

static int
GPIODirection(int pin, int dir)
{
  static const char s_directions_str[] = "in\0out";

#define DIRECTION_MAX 35
  char path[DIRECTION_MAX];
  int fd;

  snprintf(path, DIRECTION_MAX, "/sys/class/gpio/gpio%d/direction", pin);
  fd = open(path, O_WRONLY|O_CLOEXEC|O_SYNC);
  if (-1 == fd) {
    std::cerr << "Failed to open gpio direction for writing!\n" << std::endl;
    std::cout << "Path: " << path << std::endl;
    return -1;
  }

  if (-1 == write(fd, &s_directions_str[IN == dir ? 0 : 3], IN == dir ? 2 : 3)) {
    std::cerr << "Failed to set direction!" << std::endl;
    return -1;
  }

  close(fd);
  return 0;
}

static int
GPIORead(int pin)
{
#define VALUE_MAX 30
  char path[VALUE_MAX];
  char value_str[3];
  int fd;

  snprintf(path, VALUE_MAX, "/sys/class/gpio/gpio%d/value", pin);
  fd = open(path, O_RDONLY|O_CLOEXEC|O_SYNC);
  if (-1 == fd) {
    std::cerr << "Failed to open gpio value for reading!" << std::endl;
    return -1;
  }
  if (-1 == read(fd, value_str, 3)) {
    std::cerr << "Failed to read value!" << std::endl;
    return -1;
  }

  close(fd);

  return(atoi(value_str));
}

static int
GPIOWrite(int pin, int value)
{
  static const char s_values_str[] = "01";

  char path[VALUE_MAX];
  int fd;

  snprintf(path, VALUE_MAX, "/sys/class/gpio/gpio%d/value", pin);
  fd = open(path, O_WRONLY|O_CLOEXEC|O_SYNC);
  if (-1 == fd) {
    std::cerr << "Failed to open gpio value for writing!" << std::endl;
    return -1;
  }
  if (1 != write(fd, &s_values_str[LOW == value ? 0 : 1], 1)) {
    std::cerr << "Failed to write value!" << std::endl;
    return -1;
  }

  close(fd);

  return 0;
}

int main(int argc, char *argv[])
{
  using namespace std::chrono_literals;

  int repeat = 10;

  if (-1 == GPIOExport(POUT))
    return 1;

  std::this_thread::sleep_for(500ms); // It can take like 250 ms for the direction file to be available!
  if (-1 == GPIODirection(POUT, OUT))
    return 2;

  do {
    if (-1 == GPIOWrite(POUT, repeat % 2))
      return 3;

    std::this_thread::sleep_for(500ms);
  } while (repeat--);

  if (-1 == GPIOUnexport(POUT))
    return 4;

  return 0;
}
