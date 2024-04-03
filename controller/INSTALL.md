# Installation instructions

Either leave the Arduino Nano disconnected from the board, and
connect it through its USB port, or use a USB-to-TTL adapter to
connect with the Arduino Nano mounted.

You'll need to know which type of Arduino Nano board you have:

- `nanoatmega328`: for older Arduino Nanos that use the old
  firmware. I found that many Arduino Nano knockoffs use this
  firmware. I use an Elegoo board that uses this firmware.
- `nanoatmega328new`: for newer Arduino Nanos
- `nano_every`: for the Arduino Nano Every

You may need to adjust the `upload_port` property if PlatformIO
does not detect it properly. For example, on my Linux machine I
need to add `upload_port = /dev/ttyUSB0` to `platformio.ini`.

## Using PlatformIO IDE

You can find
[installation instructions for PlatformIO IDE on their website](https://platformio.org/install/ide).

Follow these steps:

- Open the project in PlatformIO IDE
- Open the PlatformIO tab
- Under "PROJECT TASKS" expand the appropriate folder for your
  hardware.
- Expand "General" as well if it's not expanded.
- _If you have the Arduino Nano mounted on the board:_ press the
  reset button and leave it pressed in.
- Click "Upload"
- _If you have the Arduino Nano mounted on the board:_ release
  the reset button when the terminal says something like
  `Uploading .../firmware.hex`.

The output should say SUCCESS. You may need to try again if you
were unsuccessful.

## Using PlatformIO Core

You can find
[installation instructions for PlatformIO Core on their website](https://platformio.org/install/cli).

Follow these steps:

- `cd` to the project directory
- _If you have the Arduino Nano mounted on the board:_ press the
  reset button and leave it pressed in.
- Run `pio run -e yourhardware -t upload`, where `yourhardware`
  is one of `nanoatmega328`, `nanoatmega328new`, or
  `nano_every`.
- _If you have the Arduino Nano mounted on the board:_ release
  the reset button when the output says something like
  `Uploading .../firmware.hex`.

The output should say SUCCESS. You may need to try again if you
were unsuccessful.