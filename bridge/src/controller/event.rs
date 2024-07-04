use crate::controller::event::Event::{FallingEdge, RisingEdge};
use thiserror::Error;

const FEDGE_EVENT: u8 = 0b0010_0000;
const REDGE_EVENT: u8 = 0b0110_0000;
const EVENT_TYPE_MASK: u8 = 0b1110_0000;
const INPUT_MASK: u8 = 0b0001_1111;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Event {
    RisingEdge(u8),
    FallingEdge(u8),
}

#[derive(Error, Debug, Eq, PartialEq)]
#[error("Error decoding event")]
pub struct EventDecodeError;

#[derive(Error, Debug, Eq, PartialEq)]
#[error("Input out of range")]
pub struct InputOutOfRange;

impl Event {
    pub fn redge(input: u8) -> Result<Self, InputOutOfRange> {
        let masked_input = input & INPUT_MASK;
        if masked_input == input {
            Ok(RisingEdge(input))
        } else {
            Err(InputOutOfRange)
        }
    }

    pub fn fedge(input: u8) -> Result<Self, InputOutOfRange> {
        let masked_input = input & INPUT_MASK;
        if masked_input == input {
            Ok(FallingEdge(input))
        } else {
            Err(InputOutOfRange)
        }
    }
}

impl TryFrom<u8> for Event {
    type Error = EventDecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Event::*;
        let event_type = value & EVENT_TYPE_MASK;
        let input = value & INPUT_MASK;
        match event_type {
            REDGE_EVENT => Ok(RisingEdge(input)),
            FEDGE_EVENT => Ok(FallingEdge(input)),
            _ => Err(EventDecodeError),
        }
    }
}

impl From<&Event> for u8 {
    fn from(value: &Event) -> Self {
        use Event::*;
        match value {
            RisingEdge(input) => REDGE_EVENT | input,
            FallingEdge(input) => FEDGE_EVENT | input,
        }
    }
}
