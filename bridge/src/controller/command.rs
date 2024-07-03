use thiserror::Error;
use crate::controller::command::Command::{Toggle, On, Off};

const COMMAND_TYPE_MASK: u8 = 0b1110_0000;
const OUTPUT_MASK: u8 = 0b0001_1111;
const REFRESH_COMMAND: u8 = 0b0010_0000;
const TOGGLE_COMMAND: u8 = 0b0100_0000;
const OFF_COMMAND: u8 = 0b1000_0000;
const ON_COMMAND: u8 = 0b1100_0000;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Command {
    Refresh,
    Toggle(u8),
    Off(u8),
    On(u8),
}

#[derive(Error, Debug, Eq, PartialEq)]
#[error("Output out of range")]
pub struct OutputOutOfRange;

#[derive(Debug, Error, Eq, PartialEq)]
#[error("Error decoding command")]
pub(crate) struct CommandDecodeError;

impl Command {
    pub fn refresh() -> Self {
        Command::Refresh
    }

    pub fn toggle(output: u8) -> Result<Self, OutputOutOfRange> {
        let masked_output = output & OUTPUT_MASK;
        if masked_output == output {
            Ok(Toggle(output))
        } else {
            Err(OutputOutOfRange)
        }
    }

    pub fn on(output: u8) -> Result<Self, OutputOutOfRange> {
        let masked_output = output & OUTPUT_MASK;
        if masked_output == output {
            Ok(On(output))
        } else {
            Err(OutputOutOfRange)
        }
    }

    pub fn off(output: u8) -> Result<Self, OutputOutOfRange> {
        let masked_output = output & OUTPUT_MASK;
        if masked_output == output {
            Ok(Off(output))
        } else {
            Err(OutputOutOfRange)
        }
    }
}

impl TryFrom<u8> for Command {
    type Error = CommandDecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let command_type = value & COMMAND_TYPE_MASK;
        let output = value & OUTPUT_MASK;
        use Command::*;
        match command_type {
            0x20 => Ok(Refresh),
            0x40 => Ok(Toggle(output)),
            0x80 => Ok(Off(output)),
            0xC0 => Ok(On(output)),
            _ => Err(CommandDecodeError {}),
        }
    }
}

impl From<&Command> for u8 {
    fn from(command: &Command) -> Self {
        use Command::*;
        match command {
            Refresh => REFRESH_COMMAND,
            Toggle(output) => TOGGLE_COMMAND | output,
            Off(output) => OFF_COMMAND | output,
            On(output) => ON_COMMAND | output,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_command() {
        use crate::controller::command::Command::*;
        let refresh: u8 = (&Refresh).into();
        assert_eq!(refresh, 0x20);
        assert_eq!(Ok(Refresh), refresh.try_into());
        let toggle: u8 = (&Toggle(1)).into();
        assert_eq!(toggle, 0x41);
        assert_eq!(Ok(Toggle(1)), toggle.try_into());
        let off: u8 = (&Off(2)).into();
        assert_eq!(off, 0x82);
        assert_eq!(Ok(Off(2)), off.try_into());
        let on: u8 = (&On(3)).into();
        assert_eq!(on, 0xC3);
        assert_eq!(Ok(On(3)), on.try_into());
    }
}
