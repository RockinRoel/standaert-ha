# The SHAL virtual machine

The virtual machine's state is described by:

- A stack of bools (of max depth 64) (encoded as a `uint64_t`)
- `input_old`: the previous input state, for 32 inputs (encoded as a `uint32_t`), **readonly**
- `input_new`: the current input state, for 32 inputs (encoded as a `uint32_t`), **readonly**
- `output_old`: the previous output state, for 32 outputs (encoded as a `uint32_t`), **readonly**
- `output_new`: the new output state, for 32 outputs (encoded as a `uint32_t`)
- An instruction counter

The instruction counter *always* advances one instruction forward, there are no jumps. We're using
the bool stack to enable or disable branches instead!

## Instructions

### Actions

- `SET [LOW/HIGH] [NUMBER]`
    - Only executed if the stack is empty or is all ones
    - Sets output to `LOW` or `HIGH`
    - 6 bits (1 bit for `LOW/HIGH`, 5 for number of output)
- `TOGGLE [NUMBER]`
    - Only executed if the stack is empty or is all ones
    - Toggles output
    - 5 bits (number of output)

### Tests

- `ON [REDGE/FEDGE] [NUMBER]`
    - `REDGE` (rising edge):
        - Pushes 1 (true) on stack if `input_old[number]` is `LOW`, and `input_new[number]` is `HIGH`
        - Pushes 0 (false) on stack otherwise
    - `FEDGE` (falling edge):
        - Pushes 1 (true) on stack if `input_old[number]` is `HIGH`, and `input_new[number]` is `LOW`
        - Pushes 0 (false) on stack otherwise
    - 6 bits (1 bit for `REDGE/FEDGE`, 5 for number of input)
- `IF [LOW/HIGH] [INPUT/OUTPUT] [IS/WAS] [NUMBER]`:
    - Checks value of input or output, pushes result on stack
    - Depending on `INPUT/OUTPUT` and `IS/WAS` we'll use a different state variable:
        - `INPUT` `WAS`: `input_old`
        - `INPUT` `IS`: `input_new`
        - `OUTPUT` `WAS`: `output_old`
        - `OUTPUT` `IS`: `output_new`
    - 8 bits (1 bit for `LOW/HIGH`, 1 for `INPUT/OUTPUT`, 1 for `IS/WAS`, 5 for number)

### Boolean stack modifiers

- `AND`:
    - Pops 2 values (`v1` and `v2`) off the stack, and pushes 1 on the stack iff `v1 = 1` and `v2 = 1`, otherwise pushes 0
- `OR`:
    - Pops 2 values (`v1` and `v2`) off the stack, and pushes 1 on the stack iff `v1 = 1` or `v2 = 1`, otherwise pushes 0
- `XOR`:
    - Pops 2 values (`v1` and `v2`) off the stack, and pushes 1 on the stack iff `v1 = 1` xor `v2 = 1`, otherwise pushes 0
- `NOT`:
    - Flips the topmost bit of the stack

### Control statements

- `POP`:
    - Pops the topmost value off the stack
- `END`
    - The end of the program

## Binary encoding

All instructions that take an input/output number are 2 bytes,
other instructions are one byte.

### Two byte instructions

Two byte instructions:

- First byte starts with `10`
- Second byte starts with `11` and indicates the input or output:  
  `110N NNNN`

First byte of instructions:

- SET: `1000 000V`: set output to `V` (`0` for `LOW`, `1` for `HIGH`)
- TOGGLE: `1000 0010`: toggle output
- ON: `1000 010X`, check input: `X` is `0` for `FEDGE` (falling edge), 1 for `REDGE` (rising edge)
- IF: `1000 1XYZ`, check input: `X` is `0` for `WAS`, `1` for `IS`; `Y` is `0` for `INPUT`, `1` for `OUTPUT`;
  `Z` is `0` for `LOW`, `1` for `HIGH`

### Single byte instructions

Single byte instructions start with `0`.

- AND: `0000 0001`
- OR: `0000 0010`
- XOR: `0000 0011`
- NOT: `0000 0100`
- POP: `0000 0101`
- END: `0000 0000`
