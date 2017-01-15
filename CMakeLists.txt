project(standaert-ha)
cmake_minimum_required(VERSION 2.8)

SET(WT_PREFIX "/usr" CACHE PATH "The Wt installation prefix")
SET(BOOST_PREFIX "/usr" CACHE PATH "The Boost installation prefix")
SET(ZLIB_PREFIX "/usr" CACHE PATH "The zlib installation prefix")
SET(SSL_PREFIX "/usr" CACHE PATH "The OpenSSL installation prefix")

link_directories(
  "${WT_PREFIX}/lib"
  "${BOOST_PREFIX}/lib"
  "${ZLIB_PREFIX}/lib"
  "${SSL_PREFIX}/lib"
)

include_directories(
  "${WT_PREFIX}/include"
  "${BOOST_PREFIX}/include"
)

add_definitions(-std=c++14)

set(SRC_LIST
  main.cpp
  config/Light.cpp
  config/LightGroup.cpp
  config/conditions.cpp
)

add_executable(${PROJECT_NAME} ${SRC_LIST})
target_link_libraries(${PROJECT_NAME}
  debug wthttpd optimized wthttp
  debug wtd optimized wt
  z ssl crypto
  boost_system boost_filesystem boost_program_options boost_thread)

set(I2C_TEST_SRC_LIST
  i2c_test.cpp
  hardware/mcp23017.cpp
)
add_executable(i2c_test ${I2C_TEST_SRC_LIST})

set(GPIO_TEST_SRC_LIST
  gpio_test.cpp
)
add_executable(gpio_test ${GPIO_TEST_SRC_LIST})
