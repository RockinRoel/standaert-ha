# Standaert Home Automation
Home automation system used at my parents' house.

This project is subdivided into three parts:

| component    | status | description |
| ------------ | ------ | ----------- |
| `pcb`        | In use (since 2019) | The KiCAD schematics and PCB layout used for the hardware |
| `controller` | In use (since 2019) | Software for the controller that is placed on the PCB. This software is written in C++ and uses PlatformIO to target the Arduin Nano. |
| `gateway`    | WIP    | Software that bridges the serial interface of the controller and home automation system software like Home Assistant. The gateway is written in Rust. |