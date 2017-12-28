#include <Hardware/I2CBusDevice.hpp>
#include <Hardware/LinuxI2CBus.hpp>
#include <Hardware/MCP23017.hpp>

#include <chrono>
#include <iomanip>
#include <iostream>
#include <thread>

using namespace StandaertHA::Hardware;
using namespace std::chrono_literals;

int main(int argc, char *argv[])
{
  MCP23017 ioexp{
    std::make_shared<I2CBusDevice>(
          std::make_shared<LinuxI2CBus>("/dev/i2c-1"),
          0x20)
  };
  ioexp.write(MCP23017::register_t::IODIRA, 0xFE);
  auto data = ioexp.read(MCP23017::register_t::IODIRA);
  std::cout << "Read 0x" << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(data) << std::endl;

  while (true) {
    ioexp.write(MCP23017::register_t::GPIOA, 0x01);
    data = ioexp.read(MCP23017::register_t::GPIOA);
    std::cout << "Read 0x" << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(data) << std::endl;
    std::this_thread::sleep_for(500ms);
    ioexp.write(MCP23017::register_t::GPIOA, 0x00);
    data = ioexp.read(MCP23017::register_t::GPIOA);
    std::cout << "Read 0x" << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(data) << std::endl;
    std::this_thread::sleep_for(500ms);
  }
}
