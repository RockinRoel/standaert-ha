project(standaert-ha)
cmake_minimum_required(VERSION 2.8)

SET(WT_PREFIX "/home/roel/work/wt4/install/debug")
SET(BOOST_PREFIX "/usr")

link_directories(
  "${WT_PREFIX}/lib"
  "${BOOST_PREFIX}/lib"
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
target_link_libraries(${PROJECT_NAME} wtd wthttpd)
