use crate::controller::program_header::ProgramHeaderDecodeError::{
    IncorrectHeaderSize, IncorrectPrefix,
};
use thiserror::Error;

pub const PROGRAM_HEADER_LENGTH: usize = 8;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProgramHeader {
    pub length: u16,
    pub crc: u16,
}

#[derive(Error, Debug, Eq, PartialEq)]
pub(crate) enum ProgramHeaderDecodeError {
    #[error("Incorrect header size: should be 8, was {actual_size}")]
    IncorrectHeaderSize { actual_size: usize },
    #[error(
        "Header does not start with \"SHAL\" ([83, 72, 65, 76]), but starts with {actual_prefix:?}"
    )]
    IncorrectPrefix { actual_prefix: Vec<u8> },
}

impl TryFrom<&[u8]> for ProgramHeader {
    type Error = ProgramHeaderDecodeError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(IncorrectHeaderSize {
                actual_size: value.len(),
            });
        }
        let prefix = &value[0..4];
        if &prefix != b"SHAL" {
            return Err(IncorrectPrefix {
                actual_prefix: prefix.into(),
            });
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

#[cfg(test)]
mod tests {
    use crate::controller::program_header::ProgramHeaderDecodeError::{
        IncorrectHeaderSize, IncorrectPrefix,
    };
    use crate::controller::program_header::{
        ProgramHeader, ProgramHeaderDecodeError, PROGRAM_HEADER_LENGTH,
    };

    #[test]
    fn test_serialize() {
        let header = ProgramHeader {
            length: 1000,
            crc: 2000,
        };
        let serialized: [u8; PROGRAM_HEADER_LENGTH] = (&header).into();
        assert_eq!(
            &[b'S', b'H', b'A', b'L', 0x03, 0xE8, 0x07, 0xD0],
            &serialized,
        )
    }

    #[test]
    fn test_deserialize() {
        let serialized = [b'S', b'H', b'A', b'L', 0x03, 0xE8, 0x07, 0xD0];
        let header: Result<ProgramHeader, ProgramHeaderDecodeError> = (&serialized[..]).try_into();
        assert_eq!(
            &Ok(ProgramHeader {
                length: 1000,
                crc: 2000,
            }),
            &header
        )
    }

    #[test]
    fn test_deserialize_incorrect_length() {
        let serialized = [b'S', b'H', b'A', b'L', 1, 2, 3];
        let result: Result<ProgramHeader, ProgramHeaderDecodeError> = (&serialized[..]).try_into();
        assert_eq!(&Err(IncorrectHeaderSize { actual_size: 7 }), &result);
        assert_eq!(
            "Incorrect header size: should be 8, was 7",
            result.unwrap_err().to_string()
        )
    }

    #[test]
    fn test_deserialize_wrong_prefix() {
        let serialized = [1, 2, 3, 4, 5, 6, 7, 8];
        let result: Result<ProgramHeader, ProgramHeaderDecodeError> = (&serialized[..]).try_into();
        assert_eq!(
            &Err(IncorrectPrefix {
                actual_prefix: vec![1, 2, 3, 4],
            }),
            &result
        );
        assert_eq!(
            "Header does not start with \"SHAL\" ([83, 72, 65, 76]), but starts with [1, 2, 3, 4]",
            result.unwrap_err().to_string()
        )
    }
}
