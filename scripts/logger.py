#!/usr/bin/env python3

# Needs pyserial module!
# And crc16!

import crc16
import serial

ser = serial.Serial()
ser.baudrate = 9600
ser.port = '/dev/ttyUSB0'
ser.open()

SLIP_END = 0xC0
SLIP_ESC = 0xDB
SLIP_ESC_END = 0xDC
SLIP_ESC_ESC = 0xDD

def slip_decode(buf):
    out = bytearray()
    i = 0
    while i < len(buf):
        if buf[i] == SLIP_ESC:
            i += 1
            if buf[i] == SLIP_ESC_END:
                out += bytes([SLIP_END])
            elif buf[i] == SLIP_ESC_ESC:
                out += bytes([SLIP_ESC])
        else:
            out.append(buf[i])
        i += 1
    return out

def log_event(ev):
    if ev & 0x40 == 0:
        return
    button = int(ev) & 0x1F;
    if ev & 0x80 == 0:
        print('Press start {}'.format(button))
    else:
        print('Press end {}'.format(button))

def log_out_state(state):
    print('{:02X}{:02X}{:02X}{:02X}'.format(state[0],state[1],state[2],state[3]))

buf = bytearray()
while True:
    data = ser.read()
    for b in data:
        if b == SLIP_END:
            if len(buf) >= 3:
                out = slip_decode(buf)
                calc_crc = crc16.crc16xmodem(bytes(out[:-2]))
                send_crc = (int(out[-2]) << 8) + int(out[-1])
                if calc_crc != send_crc:
                    print('crc fail!')
                else:
                    for b in out[:32]:
                        log_event(b)
                    out_state = out[32:36]
                    if len(out_state) == 4:
                        log_out_state(out_state)
            buf.clear()
        else:
            buf.append(b)

ser.close()
