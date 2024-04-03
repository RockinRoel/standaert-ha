const TYPE_MASK: u8 = 0xE0;
const OUTPUT_MASK: u8 = 0x1F;
const TOGGLE: u8 = 0x40;
const OFF: u8 = 0x80;
const ON: u8 = 0xC0;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Command {
    None,
    Refresh,
    Toggle(u8),
    Off(u8),
    On(u8),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommandDecodeError;

impl TryFrom<u8> for Command {
    type Error = CommandDecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let command_type = value & TYPE_MASK;
        let output = value & OUTPUT_MASK;
        use Command::*;
        match command_type {
            0x00 => Ok(None),
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
            None => 0x00,
            Refresh => 0x20,
            Toggle(output) => TOGGLE | (output & OUTPUT_MASK),
            Off(output) => OFF | (output & OUTPUT_MASK),
            On(output) => ON | (output & OUTPUT_MASK),
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
