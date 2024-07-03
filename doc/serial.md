# Serial communication specification

## Communicating parties

- **Controller**: the Arduino Nano device mounted on the board
- **Host**: the system that it's connected to, like a PC or
  single board computer like a Raspberry Pi.

## Serial configuration

- baudrate: 115200 baud
- 8-N-1:
  - 8 data bits
  - no parity bit
  - 1 stop bit

## Message encoding

Messages are sent SLIP encoded.

Every message starts with 2 CRC bytes (a 16-bit XMODEM CRC
in big-endian order), and is followed by a byte indicating the
message type:

- `u`: Update message (controller to host, contains output state
  and input events)
- `c`: Command message (host to controller, contains commands from
  the host for the controller)
- `F`: Failure message (controller to host, contains a UTF-8 encoded
  error message)
- `I`: Info message (controller to host, contains a UTF-8 encoded
  info message)
- `s`: Program start (host to controller, indicates that the
  host wants to upload a new SHAL bytecode program to the controller)
- `S`: Program start ack (controller to host, acknowledges that
  the host is ready to receive a new SHAL bytecode program)
- `d`: Program data (host to controller, contains a SHAL bytecode
  program chunk)
- `e`: Program end (host to controller, contains the last SHAL
  bytecode program chunk)
- `E`: Program end ack (controller to host, indicates that the
  controller has received the program)

All invalid messages, including partial messages,
messages with an incorrect CRC, or unrecognized message types
will be ignored.

A message type of `00` (the null byte) is not used.

The maximum message size (including 3-byte header) is 128 bytes.

## Controller to host

There are three kinds of messages that will be sent from the
controller to the host:

- `u`: update message
- `S`: program start ack
- `E`: program end ack

### Update message

If there's an event and/or an update in output state the
controller will send an update message:

- the **state** of the outputs (4 bytes, big endian)
- the input events

#### State

The state is a 32-bit integer, where the least significant
bit corresponds to the state of output 1 (1 for high, 0 for low)
and the most significant bit corresponds to the state of output 32.
Note that on the board output 1 is the rightmost output.

The state reflects the state after running the SHAL program.

#### Input event

Input events are encoded as follows:

```
TTTIIIII

TTT: event type
IIIII: input id

e.g.
00100010 : third input (input id 2, counting from 0) falling edge
01100010 : third input (input id 2, counting from 0) rising edge
```

The type (`T`) bits indicate falling edge (`001`) or rising edge
(`011`). The 5 least significant bits indicate the input id.
`00000` is input 1, `00001` is input 2, and so on.
Note that on the board input 1 is the leftmost input.

Type `000` is not used (this is so that zeroed out memory can not
interpreted as an event).

Examples:

- `00100010`: third input (input id 2) falling edge
- `01100101`: sixth input (input id 5) rising edge

### Program start ack

The program start ack message contains the program header that was
sent in the program start message.

### Program end ack

The program end ack message contains the program header of the
received program. This includes the CRC and length calculated by the
controller. If the CRC and/or length are incorrect, the program
will not be accepted, and the host should retry the upload.

## Host to controller

There are four kinds of messages that will be sent from the
controller to the host:

- `c`: command message
- `s`: program start
- `d`: program data
- `e`: program end

### Command message

All commands are processed in the order that they are received, so
if a message first contains a command to set output 1 to `HIGH`, and
then a command to set output 1 to `LOW`, the result will be that
output 1 is `LOW`.

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

- `001`: **Refresh**: forces the controller to send an update
  message to the host. This is used when the host initiates
  communication to get the initial state (the host can be restarted
  independently of the controller).
- `010`: **Toggle** the output with the given id.
- `100`: **Off**: sets the output with the given id to `LOW`.
- `110`: **On**: sets the output with the given id to `HIGH`.

Type `000` is not used (this is so that zeroed out memory can not
interpreted as a command).

The remaining values (`011`, `101`, and `111`) currently have no
function and are ignored by the controller.

### Program start

This message contains the program header, and indicates to
the controller that the host wants to initiate program upload.

Once acknowledged with a program start ack message, the host
may start sending the program.

### Program data

This message contains program data.

### Program end

This message contains program data, and indicates to the controller
that the host is done sending the program.

Once acknowledged with a program end ack message, the host knows
that upload was successful.