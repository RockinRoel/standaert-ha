add_executable(mcp23017_blink mcp23017_blink.cpp)
target_include_directories(mcp23017_blink PUBLIC ../src)
target_link_libraries(mcp23017_blink standaert_ha)
