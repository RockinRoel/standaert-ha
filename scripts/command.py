import crc16
import serial

SLIP_END = 0xC0
SLIP_ESC = 0xDB
SLIP_ESC_END = 0xDC
SLIP_ESC_ESC = 0xDD

def slip_encode(buf):
    out = bytearray()
    out += bytes([SLIP_END])
    for b in buf:
        if b == SLIP_ESC:
            out += bytes([SLIP_ESC, SLIP_ESC_ESC])
        elif b == SLIP_END:
            out += bytes([SLIP_ESC, SLIP_ESC_END])
        else:
            out.append(b)
    out += bytes([SLIP_END])
    return bytes(out)

def append_crc(buf):
    out = bytearray()
    out += buf
    crc = crc16.crc16xmodem(buf)
    out += bytes([(crc >> 8) & 0xFF, crc & 0xFF])
    return bytes(out)

def send_commands(ser, commands):
    packet = slip_encode(append_crc(bytes(commands)))
    ser.write(packet)
