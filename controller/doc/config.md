# Configuration

Currently the configuration is hardcoded in the file
[`include/config.hpp`](include/config.hpp).

This file contains two functions:

- `handleEvent`
- `postprocess`

## `handleEvent`

The `handleEvent` function takes a `DSL::Context&`, which has three
functions that return an `Event` object:

- `on(ButtonEvent)`: on an arbitrary button event
- `onPressStart(uint8_t)`: when the given button is pressed.
  We count from `0`, so button 1 on the board is `0`, and button
  32 is `31`.
- `onPressEnd(uint8_t)`: when the given button is released.

The `Event` object has the following functions:

- `toggle(uint8_t)`: toggle the given output. We count from `0`, so
  output 1 on the board is `0`, and output 32 is `31`.
- `turnOn(uint8_t)`: turns the given output on.
- `switchOff(uint8_t)`: turns the given output off.

These functions return an `Event` object so they can be chained.

### Examples

```cpp
constexpr inline void handleEvent(DSL::Context &c)
{
  // When button 1 is pressed, toggle output 1
  c.onPressStart(0).toggle(0);
  // When button 2 is pressed, switch output 2 off
  c.onPressStart(1).switchOff(1);
  // When button 3 is pressed, turn output 2 on
  c.onPressStart(2).turnOn(1);
  // When button 4 is pressed, turn output 3 on...
  c.onPressStart(3).turnOn(2);
  // ...and when it is released, switch it off
  c.onPressEnd(3).switchOff(2);
  // When button 5 is pressed, toggle both output 4 and 5:
  c.onPressStart(4)
    .toggle(3)
    .toggle(4);
}
```

## `postprocess`

The `postprocess` function can be used to do arbitrary logic
after the event processing. For example, if we have a light in the
hallway downstairs and a light in the hallway upstairs, and
another light on the stairway, we can make it so that the stairway
light is on if the light upstairs or the light downstairs is on.

The `postprocess` function takes the `output` as a `uint32_t&`. We
can use the `getBit` and `setBit` functions to do our `OR` logic:

```cpp
constexpr inline void postprocess(uint32_t &output)
{
  // If both output 1 and output 4 are HIGH
  if (getBit(output, 0) == HIGH ||
      getBit(output, 3) == HIGH) {
    // Set output 6 to HIGH
    setBit(output, 5, HIGH);
  } else {
    // Otherwise set output 6 to LOW
    setBit(output, 5, LOW);
  }
}
```