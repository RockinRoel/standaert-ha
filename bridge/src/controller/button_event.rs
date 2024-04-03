const VALID: u8 = 0x40;
const PRESS_END : u8 = 0x80;
const MASK: u8 = 0x1F;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum ButtonEvent {
    Invalid,
    PressStart(u8),
    PressEnd(u8),
}

impl From<u8> for ButtonEvent {
    fn from(value: u8) -> Self {
        use ButtonEvent::*;
        if value & VALID == 0 {
            Invalid
        } else {
            if value & PRESS_END == 0 {
                PressStart(value & MASK)
            } else {
                PressEnd(value & MASK)
            }
        }
    }
}

impl From<&ButtonEvent> for u8 {
    fn from(value: &ButtonEvent) -> Self {
        use ButtonEvent::*;
        match value {
            Invalid => 0,
            PressStart(button) => VALID | (button & MASK),
            PressEnd(button) => VALID | PRESS_END | (button & MASK),
        }
    }
}
