# SHAL language description

Programs are subdivided in a declaration section and a program section.

## Comments

Comments start with `//`, and end at the end of a line.

## Declaration section

In this section entities are declared, in the following form:

```
entity [ID] = [input/output] [NUMBER];
```

Where:

- `[ID]` is an alphabetical ASCII character, following by 0 or more alphanumeric ASCII characters or underscores
- `[input/output]` is either `input` or `output`, indicating whether we're declaring an input or output
- `[NUMBER]` is a positive number (starting from `0`) without leading zeroes

### Examples

```
entity button_bathroom = input 3;
entity light_bathroom = output 3;
```

## Program section

The program can contain event blocks, condition blocks, or actions.

The program is executed in a perpetual loop.

### Event block

An event block is a test whether an input went from low to high (rising edge, or `redge`)
or from high to low (falling edge, or `fedge`):

```
on [redge/fedge] [INPUT] {
  [BODY]
}
```

Where:

- `[redge/fedge]` is either `redge` or `fedge`, indicating whether we're testing
  for rising edge or falling edge
- `[INPUT]` is either en entity ID corresponding to an input, or `input [NUMBER]`,
  where `[NUMBER]` is a positive number (starting from `0`) without leading zeroes
- `[BODY]` is any sequence of actions or condition blocks, that are evaluated when the
  event occurs.

Note: unlike condition blocks, event blocks have to be top-level and thus can not be nested.
There is no technical reason for this, it just encourages a certain structure to programs.

### Condition block

A condition block is a block of the form:

```
if [CONDITION] {
  [BODY]
} [ELSE]
```

Where `[CONDITION]` is the condition that is being tested,
`[BODY]` is any sequence of actions or condition blocks, that are evaluated when the
condition is satisfied.

The `[ELSE]` is optional and can either be `else` followed by another condition block,
or `else { [BODY] }`.

#### Condition

Conditions can check either the previous state or the new state of an input or output.

```
[INPUT/OUTPUT] [is/was] [high/low]
```

- `[INPUT/OUTPUT]` is either en entity ID, `input [NUMBER]`, or `output [NUMBER]`,
  where `[NUMBER]` is a positive number (starting from `0`) without leading zeroes
- `[is/was]` is either `is` or `was`, where `is` checks the new state and `was` checks
  the previous state.
 
  For outputs, `was` corresponds to the state of the outputs before the current 
  loop, and `is` checks the state that the output is currently set at, and may have
  changed through a previous statement in the loop.

  For inputs, `was` corresponds to the state of the input at the previous loop,
  and `is` corresponds to the state of the input now.

  Note: prefer using events rather than complicated conditions, e.g. use
  `on redge input 0` instead of `if input 0 was low and input 0 is high`.
- `[high/low]` is either `high` or `low`, indicating whether we are checking if the
  input/output is/was high or low.

Boolean operations can be applied to conditions to form new conditions:

- `( [CONDITION] )`
- `[CONDITION] and [CONDITION]`
- `[CONDITION] or [CONDITION]`
- `[CONDITION] xor [CONDITION]`
- `not [CONDITION]`

There are no precedence rules. The following is illegal code:

```
if foo is high and bar is low or baz is high { // ILLEGAL CODE
```

Use brackets instead:

```
if (foo is high and bar is low) or baz is high {
```

### Action

Actions can either toggle or set the value of an output.

- `toggle [OUTPUT];`
- `set [OUTPUT] [low/high];`

Where:

- `[OUTPUT]` is either en entity ID corresponding to an output, or `output [NUMBER]`,
  where `[NUMBER]` is a positive number (starting from `0`) without leading zeroes
- `[low/high]` is either `low` or `high` depending on the desired state of the output

### Examples

Here's a nonsensical example to demonstrate the language:

```
on redge input 1 {
  if output 1 was high and output 2 is high {
    toggle output 3;
  } else if output 1 is low {
    set output 1 high;
  } else {
    set output 2 low;
  }
}
```
