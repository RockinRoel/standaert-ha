#!/usr/bin/env python3

import command
import serial

ser = serial.Serial()
ser.baudrate = 9600
ser.port = '/dev/ttyUSB0'
ser.open()

while True:
    n = int(input("Which one to toggle? "))
    c = 0x40 | (n & 0x1F)
    command.send_commands(ser, [c])

ser.close()
