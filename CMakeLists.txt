cmake_minimum_required(VERSION 3.1)
project(StandaertHA)

set(CMAKE_CXX_STANDARD 14)

include(cmake/CheckLinuxI2C.cmake)

set(SRCS
    src/Hardware/I2CBus.cpp
    src/Hardware/I2CDevice.cpp
    src/Hardware/I2CBusDevice.cpp)

if(LINUX_I2C_ENABLED)
    set(SRCS ${SRCS} src/Hardware/LinuxI2CBus.cpp)
endif(LINUX_I2C_ENABLED)

add_library(standaert_ha ${SRCS})
