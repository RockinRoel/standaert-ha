# SHAL Bytecode

SHAL = Standaert Home Automation Language

```
000 IIIII: FEDGE on input IIIII -> output is INVERTED --> save in LEFT register
001 IIIII: REDGE on input IIIII -> output is INVERTED --> save in LEFT register
010 OOOOO: CHECK if STATE of output OOOOO is LOW --> save in LEFT register
011 OOOOO: CHECK if STATE of output OOOOO is HIGH --> save in LEFT register
100 OOOOO: SET output OOOOO LOW
101 OOOOO: SET output OOOOO HIGH
110 OOOOO: TOGGLE output OOOOO

111 00A AA: jump forward AAA + 2 instructions
111 01A AA: conditional jump forward AAA + 2 instructions

1111 000 R: invert value of register R (0 for LEFT, 1 for RIGHT)
1111 001 R: AND LEFT and RIGHT, save in register R (0 for LEFT, 1 for RIGHT)
1111 010 R: OR LEFT and RIGHT, save in register R (0 for LEFT, 1 for RIGHT)

1111 1111: do nothing
```

```
on redge 12 {
  toggle 13
}
```

may also be written as:

```
on redge 12 toggle 13
```

translates to:

```
REDGE 11
CJMP 2
TOGGLE 12
```

Default program:

```
redge 1 toggle 1
redge 2 toggle 2
redge 3 toggle 3
redge 4 toggle 4
redge 5 toggle 5
redge 6 toggle 6
redge 7 toggle 7
redge 8 toggle 8
redge 9 toggle 9
redge 10 toggle 10
redge 11 toggle 11
redge 12 toggle 12
redge 13 toggle 13
redge 14 toggle 14
redge 15 toggle 15
redge 16 toggle 16
redge 17 toggle 17
redge 18 toggle 18
redge 19 toggle 19
redge 20 toggle 20
redge 21 toggle 21
redge 22 toggle 22
redge 23 toggle 23
redge 24 toggle 24
redge 25 toggle 25
redge 26 toggle 26
redge 27 toggle 27
redge 28 toggle 28
redge 29 toggle 29
redge 30 toggle 30
redge 31 toggle 31
redge 32 toggle 32
```

translates to:

```
REDGE 0
CJMP 2
TOGGLE 0
REDGE 1
CJMP 2
TOGGLE 1
REDGE 2
CJMP 2
TOGGLE 2
REDGE 3
CJMP 2
TOGGLE 3
...
```

in bytecode:

```
001 00000
111 001 00
110 00000
001 00001
111 001 00
110 00001
001 00010
111 001 00
110 00010
001 00011
111 001 00
110 00011
```

3 * 32 bytes = 96 bytes
