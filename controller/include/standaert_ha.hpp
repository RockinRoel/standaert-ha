#pragma once

namespace StandaertHA {
    constexpr const int MODE_PIN = 10;
    constexpr const int MSB_PIN = 11;
    constexpr const int RST_IN1_PIN = 6;
    constexpr const int RST_IN2_PIN = 8;
    constexpr const int RST_OUT1_PIN = 9;
    constexpr const int RST_OUT2_PIN = 7;

    constexpr const int IN1_ADDR = 0x20;
    constexpr const int IN2_ADDR = 0x21;
    constexpr const int OUT1_ADDR = 0x22;
    constexpr const int OUT2_ADDR = 0x23;

    constexpr const int MCP23017_IODIRA = 0x00;
    constexpr const int MCP23017_IODIRB = 0x01;
    constexpr const int MCP23017_IPOLA = 0x02;
    constexpr const int MCP23017_IPOLB = 0x03;
    constexpr const int MCP23017_GPINTENA = 0x04;
    constexpr const int MCP23017_GPINTENB = 0x05;
    constexpr const int MCP23017_DEFVALA = 0x06;
    constexpr const int MCP23017_DEFVALB = 0x07;
    constexpr const int MCP23017_INTCONA = 0x08;
    constexpr const int MCP23017_INTCONB = 0x09;
    constexpr const int MCP23017_IOCON = 0x0A;
    constexpr const int MCP23017_IOCON_ALT = 0x0B;
    constexpr const int MCP23017_GPPUA = 0x0C;
    constexpr const int MCP23017_GPPUB = 0x0D;
    constexpr const int MCP23017_INTFA = 0x0E;
    constexpr const int MCP23017_INTFB = 0x0F;
    constexpr const int MCP23017_INTCAPA = 0x10;
    constexpr const int MCP23017_INTCAPB = 0x11;
    constexpr const int MCP23017_GPIOA = 0x12;
    constexpr const int MCP23017_GPIOB = 0x13;
    constexpr const int MCP23017_OLATA = 0x14;
    constexpr const int MCP23017_OLATB = 0x15;

    constexpr const unsigned long int DEBOUNCE_TIME_MILLIS = 30UL;
}