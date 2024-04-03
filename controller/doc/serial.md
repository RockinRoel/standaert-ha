# Serial communication specification

## Communicating parties

- **Controller**: the Arduino Nano device mounted on the board
- **Host**: the system that it's connected to, like a PC or
  single board computer like a Raspberry Pi.

## Serial configuration

- baudrate: 9600 baud
- 8-N-1:
  - 8 data bits
  - no parity bit
  - 1 stop bit

## Message encoding

A 16-bit XMODEM CRC is appended to every message in big-endian
order, and then SLIP encoded.

The controller will ignore partial messages or messages with an
incorrect CRC.

## Controller to host

If there's an event and/or an update in output state the
controller will send a message of exactly 38 bytes (before
SLIP encoding):

- 32 **button events** (1 byte each)
- the **state** of the outputs (4 bytes, big endian)
- XMODEM CRC (2 bytes, big endian)

### Button event

Button events are encoded as follows:

```
TV0IIIII
^^     ^
||     \- Button id
|\------- Valid
\-------- Type

e.g.
01000010 : second button press start
11000010 : second button press end
00000000 : not valid
```

Buttons with the valid (`V`) bit set to `0` are to be ignored. The
type (`T`) bit indicates press start (`0`) or press end (`1`). The
5 least significant bits indicate the button id. `00000` is button
1, `00001` is button 2, and so on. Note that on the board button 1
uses the leftmost input. The remaining bit is set to `0` and is to
be ignored.

Examples:

- `01000010`: second button press start
- `11000101`: fifth button press end
- `00000000`: not valid, to be ignored

The controller will send all button events that occurred, and pad
the remaining space with zeroes until it reaches 32 bytes.

### State

The state is a 32-bit integer, where the least significant
bit corresponds to the state of output 1 (1 for high, 0 for low)
and the most significant bit corresponds to the state of output 32.
Note that on the board output 1 is the rightmost output.

The state reflects the state after the button events were applied.

## Host to controller

The host can send commands to the controller. At most 64 commands
can be sent in one message. Messages may be of variable length.
The length is not encoded as part of the message. The controller
derives the length of the message from the SLIP encoding and
makes sure the message is complete using the CRC.

All commands are processed in the order that they are received, so
if a message first contains a command to set output 1 to `HIGH`, and
then a command to set output 1 to `LOW`, the result will be that
output 1 is `LOW`.

### Command

A command is encoded as a single byte, encoded as follows:

```
TTTIIIII
 
TTT: command type
IIIII: output id
```

The 3 most significant bits indicate the command type, and the 5
least significant bits indicate the output id.

The output id specifies which output the command affects. If the
command is not specific to an output id it is ignored. Usually
the host wil send `00000` in that case. Output 1 is encoded as
`00000`, output 2 is encoded as `00001`, and so on. Note that
output 1 is the rightmost output on the board.

The command type can be:

- `000`: None. These are not normally sent, but are used in the code
  of the controller as a default value. These commands are ignored.
- `001`: Refresh. Forces the controller to send its state to the
  host. This is used when the host initiates communication to get
  the initial state (the host can be restarted independently of the
  controller).
- `010`: Toggle the output with the given id.
- `100`: Sets the output with the given id to `LOW`.
- `110`: Sets the output with the given id to `HIGH`.

The remaining values (`011`, `101`, and `111`) currently have no
function and are ignored by the controller.