cmake_minimum_required(VERSION 3.1)
project(StandaertHA)

set(CMAKE_CXX_STANDARD 14)

set(SRCS
    src/Hardware/I2CBus.cpp
    src/Hardware/I2CDevice.cpp
    src/Hardware/LinuxI2CBus.cpp)

add_library(standaert_ha ${SRCS})
