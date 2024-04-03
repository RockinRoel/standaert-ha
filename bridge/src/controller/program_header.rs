use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

const PROGRAM_HEADER_LENGTH: usize = 8;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProgramHeader {
    length: u16,
    crc: u16,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProgramHeaderDecodeError;

impl fmt::Display for ProgramHeaderDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Program header decode error") // TODO(Roel)
    }
}

impl Error for ProgramHeaderDecodeError {
    fn description(&self) -> &str {
        "Program header decode error" // TODO(Roel)
    }
}

impl TryFrom<&[u8]> for ProgramHeader {
    type Error = ProgramHeaderDecodeError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(ProgramHeaderDecodeError {});
        }
        if &value[0..4] != b"SHAL" {
            return Err(ProgramHeaderDecodeError {});
        }
        let len_bytes = &value[4..6];
        let crc_bytes = &value[6..8];
        Ok(ProgramHeader {
            length: u16::from_be_bytes([len_bytes[0], len_bytes[1]]),
            crc: u16::from_be_bytes([crc_bytes[0], crc_bytes[1]]),
        })
    }
}

impl From<&ProgramHeader> for [u8; PROGRAM_HEADER_LENGTH] {
    fn from(value: &ProgramHeader) -> Self {
        let len_bytes = value.length.to_be_bytes();
        let crc_bytes = value.crc.to_be_bytes();
        [
            b'S',
            b'H',
            b'A',
            b'L',
            len_bytes[0],
            len_bytes[1],
            crc_bytes[0],
            crc_bytes[1],
        ]
    }
}

impl ProgramHeader {
    pub const fn header_length() -> usize {
        PROGRAM_HEADER_LENGTH
    }

    pub fn new(length: u16, crc: u16) -> Self {
        ProgramHeader { length, crc }
    }
}
