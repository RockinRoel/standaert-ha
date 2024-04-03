Has a 32 bit boolean stack

```
FEDGE:      000X XXXX -> push 1 on stack on fedge (falling edge) on input XXXXX, 0 if not
REDGE:      001I IIII -> push 1 on stack on redge (rising edge) on input XXXXX, 0 if not
STATE LOW:  010X XXXX -> push 1 on stack if state of output XXXXX is LOW
STATE HIGH: 011X XXXX -> push 1 on stack if state of output XXXXX is HIGH
SET LOW:    100X XXXX -> set output XXXXX LOW, if all bits on the stack are 1
SET HIGH:   101X XXXX -> set output XXXXX HIGH, if all bits on the stack are 1
TOGGLE:     110X XXXX -> toggle output XXXXX

OR:         1110 0000 -> OR topmost two values on stack, pop them and leave result
AND:        1110 0001 -> AND topmost two values on stack, pop them and leave result
NOT:        1110 0010 -> replace top of stack with NOT the top of stack
POP:        1110 0011 -> POP the top of the stack

NOP:        1111 1111
```

```
REDGE 00000 --> stack: 1
TOGGLE 00000
POP
```
