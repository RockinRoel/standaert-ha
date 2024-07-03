use std::string::FromUtf8Error;
use static_assertions as sa;

use crate::controller::event::{Event, EventDecodeError};
use crate::controller::command::{Command, CommandDecodeError};
use crate::controller::message::MessageDecodingError::{CrcError, SizeTooLarge, SizeTooSmall, UnknownType};
use crate::controller::program_header::{ProgramHeader, ProgramHeaderDecodeError};
use crc::{Crc, CRC_16_XMODEM};
use thiserror::Error;

const MESSAGE_HEADER_LENGTH: usize = 3;
const MIN_MESSAGE_LENGTH: usize = MESSAGE_HEADER_LENGTH;
const MAX_MESSAGE_LENGTH: usize = 128;
const MAX_MESSAGE_BODY_LENGTH: usize = MAX_MESSAGE_LENGTH - MESSAGE_HEADER_LENGTH;

sa::const_assert_eq!(MIN_MESSAGE_LENGTH, 3);
sa::const_assert_eq!(MAX_MESSAGE_BODY_LENGTH, 125);

#[derive(Error, Debug, Eq, PartialEq)]
pub(crate) enum MessageDecodingError {
    #[error("Size too small, message size should be at least {MIN_MESSAGE_LENGTH}, actual size: {actual_size}")]
    SizeTooSmall {
        actual_size: usize,
    },
    #[error("Size too large, message size should be at most {MAX_MESSAGE_LENGTH}, actual size: {actual_size}")]
    SizeTooLarge {
        actual_size: usize,
    },
    #[error("CRC error")]
    CrcError,
    #[error("Unknown message type ({type_byte})")]
    UnknownType {
        type_byte: u8,
    },
    #[error("Error decoding event")]
    EventDecodeError(#[from] EventDecodeError),
    #[error("Error decoding command")]
    CommandDecodeError(#[from] CommandDecodeError),
    #[error("Error UTF-8 decoding message")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("Error decoding program header")]
    ProgramHeaderDecodeError(#[from] ProgramHeaderDecodeError),
}

// Message:
// CRC: 2 bytes
// type: 1 byte
// body: at most 128 - 3 = 125 bytes

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Message {
    crc: u16,
    body: MessageBody,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum MessageBody {
    Update {
        outputs: u32,
        events: Vec<Event>,
    },
    Command {
        commands: Vec<Command>,
    },
    Fail {
        message: String,
    },
    Info {
        message: String,
    },
    ProgramStart {
        header: ProgramHeader,
    },
    ProgramStartAck {
        header: ProgramHeader,
    },
    ProgramData {
        code: Vec<u8>, // at most 125
    },
    ProgramEnd {
        code: Vec<u8>, // at most 125
    },
    ProgramEndAck {
        header: ProgramHeader,
    },
}

impl Message {
    pub(crate) fn new(body: MessageBody) -> Self {
        Message {
            crc: body.crc(),
            body,
        }
    }
}

impl MessageBody {
    fn start_byte(&self) -> u8 {
        use MessageBody::*;
        match self {
            Update { .. } => b'u',
            Command { .. } => b'c',
            Fail { .. } => b'F',
            Info { .. } => b'I',
            ProgramStart { .. } => b's',
            ProgramStartAck { .. } => b'S',
            ProgramData { .. } => b'd',
            ProgramEnd { .. } => b'e',
            ProgramEndAck { .. } => b'E',
        }
    }

    fn crc(&self) -> u16 {
        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let mut digest = crc.digest();
        digest.update(&[self.start_byte()]);
        use MessageBody::*;
        match self {
            Update { outputs, events } => {
                digest.update(&outputs.to_be_bytes()[..]);
                for event in events {
                    let event_byte: u8 = event.into();
                    digest.update(&[event_byte]);
                }
            }
            Command { commands } => {
                for command in commands {
                    let command_byte: u8 = command.into();
                    digest.update(&[command_byte]);
                }
            }
            Fail { message } | Info { message } => {
                digest.update(message.as_bytes())
            }
            ProgramStart { header } | ProgramStartAck { header } | ProgramEndAck { header } => {
                let header_bytes: [u8; ProgramHeader::header_length()] = header.into();
                digest.update(&header_bytes);
            }
            ProgramData { code } | ProgramEnd { code } => {
                digest.update(code);
            }
            _ => {}
        }
        digest.finalize()
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = MessageDecodingError;

    fn try_from(message_bytes: &[u8]) -> Result<Self, Self::Error> {
        // Smallest possible message is 3 bytes (CRC + zero byte)
        if message_bytes.len() < MIN_MESSAGE_LENGTH {
            return Err(SizeTooSmall {
                actual_size: message_bytes.len(),
            });
        }
        // Largest possible message is 128 bytes
        if message_bytes.len() > MAX_MESSAGE_LENGTH {
            return Err(SizeTooLarge {
                actual_size: message_bytes.len(),
            });
        }
        let read_crc = u16::from_be_bytes([message_bytes[0], message_bytes[1]]);
        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let type_byte = message_bytes[2];
        let body = &message_bytes[MESSAGE_HEADER_LENGTH..];
        let mut digest = crc.digest();
        digest.update(&[type_byte]);
        digest.update(body);
        if digest.finalize() != read_crc {
            return Err(CrcError);
        }
        match type_byte {
            b'u' if body.len() >= 4 => {
                let outputs = u32::from_be_bytes([body[0], body[1], body[2], body[3]]);
                let mut events: Vec<Event> = vec![];
                for b in &body[4..] {
                    let e = (*b).try_into()?;
                    events.push(e);
                }
                Ok(Message {
                    crc: read_crc,
                    body: MessageBody::Update { outputs, events },
                })
            }
            b'c' => {
                let mut commands: Vec<Command> = vec![];
                for b in body {
                    commands.push((*b).try_into()?);
                }
                Ok(Message {
                    crc: read_crc,
                    body: MessageBody::Command { commands },
                })
            }
            b'F' => {
                Ok(Message {
                    crc: read_crc,
                    body: MessageBody::Fail { message: String::from_utf8(body.to_vec())? },
                })
            }
            b'I' => {
                Ok(Message {
                    crc: read_crc,
                    body: MessageBody::Info { message: String::from_utf8(body.to_vec())? },
                })
            }
            b's' if body.len() == ProgramHeader::header_length() => {
                let header = body.try_into()?;
                Ok(Message {
                    crc: read_crc,
                    body: MessageBody::ProgramStart { header },
                })
            }
            b'S' if body.len() == ProgramHeader::header_length() => {
                let header = body.try_into()?;
                Ok(Message {
                    crc: read_crc,
                    body: MessageBody::ProgramStartAck { header },
                })
            }
            b'd' => Ok(Message {
                crc: read_crc,
                body: MessageBody::ProgramData { code: body.into() },
            }),
            b'e' => Ok(Message {
                crc: read_crc,
                body: MessageBody::ProgramEnd { code: body.into() },
            }),
            b'E' if body.len() == ProgramHeader::header_length() => {
                let header = body.try_into()?;
                Ok(Message {
                    crc: read_crc,
                    body: MessageBody::ProgramEndAck { header },
                })
            }
            _ => Err(UnknownType {
                type_byte,
            })
        }
    }
}

impl From<&Message> for Vec<u8> {
    fn from(message: &Message) -> Self {
        let mut result: Vec<u8> = vec![];
        result.extend_from_slice(&message.crc.to_be_bytes()[..]);
        result.push(message.body.start_byte());
        let mut body_bytes: Vec<u8> = (&message.body).into();
        result.append(&mut body_bytes);
        result
    }
}

impl From<&MessageBody> for Vec<u8> {
    fn from(body: &MessageBody) -> Self {
        match body {
            MessageBody::Update { events, outputs } => {
                let mut result = vec![];
                result.extend_from_slice(&outputs.to_be_bytes());
                for event in events {
                    result.push(event.into());
                }
                result
            }
            MessageBody::Command { commands } => {
                let mut result = vec![];
                for command in commands {
                    result.push(command.into());
                }
                result
            }
            MessageBody::Fail { message }
            | MessageBody::Info { message } => {
                message.as_bytes().into()
            }
            MessageBody::ProgramStart { header }
            | MessageBody::ProgramStartAck { header }
            | MessageBody::ProgramEndAck { header } => {
                let header_bytes: [u8; ProgramHeader::header_length()] = header.into();
                header_bytes.into()
            }
            MessageBody::ProgramData { code } | MessageBody::ProgramEnd { code } => code.clone(),
        }
    }
}

impl Message {
    pub const fn message_header_length() -> usize {
        MESSAGE_HEADER_LENGTH
    }

    pub const fn max_message_length() -> usize {
        MAX_MESSAGE_LENGTH
    }

    pub const fn max_message_body_length() -> usize {
        MAX_MESSAGE_BODY_LENGTH
    }
}

#[cfg(test)]
mod tests {
    use crate::controller::event::Event;
    use crate::controller::message::{Message, MessageBody};
    use crate::controller::program_header::ProgramHeader;

    #[test]
    fn test_update() {
        let message = Message::new(MessageBody::Update {
            outputs: 0xAABBCCDD,
            events: vec![Event::RisingEdge(1), Event::FallingEdge(2)],
        });
        let bytes: Vec<u8> = (&message).into();
        assert_eq!(
            &bytes,
            &[0xBF, 0x45, b'u', 0xAA, 0xBB, 0xCC, 0xDD, 0x61, 0x22,]
        );
        let message2 = (&bytes[..]).try_into();
        assert_eq!(Ok(message), message2);
    }

    #[test]
    fn test_program_start() {
        let header = ProgramHeader::new(0xAABB, 0xCCDD);
        let message = Message::new(MessageBody::ProgramStart { header });
        let bytes: Vec<u8> = (&message).into();
        assert_eq!(
            &bytes,
            &[0xBB, 0x06, b's', b'S', b'H', b'A', b'L', 0xAA, 0xBB, 0xCC, 0xDD],
        );
        let message2 = (&bytes[..]).try_into();
        assert_eq!(Ok(message), message2);
    }

    #[test]
    fn test_program_start_ack() {
        let header = ProgramHeader::new(0xAABB, 0xCCDD);
        let message = Message::new(MessageBody::ProgramStartAck { header });
        let bytes: Vec<u8> = (&message).into();
        assert_eq!(
            &bytes,
            &[0x1C, 0xFA, b'S', b'S', b'H', b'A', b'L', 0xAA, 0xBB, 0xCC, 0xDD],
        );
        let message2 = (&bytes[..]).try_into();
        assert_eq!(Ok(message), message2);
    }

    #[test]
    fn test_program_data() {
        let message = Message::new(MessageBody::ProgramData { code: vec![0] });
        let bytes: Vec<u8> = (&message).into();
        assert_eq!(&bytes, &[0xC7, 0xEE, b'd', 0x00],);
        let message2 = (&bytes[..]).try_into();
        assert_eq!(Ok(message), message2);
    }

    #[test]
    fn test_program_end() {
        let message = Message::new(MessageBody::ProgramEnd { code: vec![0] });
        let bytes: Vec<u8> = (&message).into();
        assert_eq!(&bytes, &[0xF4, 0xDF, b'e', 0x00],);
        let message2 = (&bytes[..]).try_into();
        assert_eq!(Ok(message), message2);
    }

    #[test]
    fn test_program_end_ack() {
        let header = ProgramHeader::new(0xAABB, 0xCCDD);
        let message = Message::new(MessageBody::ProgramEndAck { header });
        let bytes: Vec<u8> = (&message).into();
        assert_eq!(
            &bytes,
            &[0x15, 0x8C, b'E', b'S', b'H', b'A', b'L', 0xAA, 0xBB, 0xCC, 0xDD],
        );
        let message2 = (&bytes[..]).try_into();
        assert_eq!(Ok(message), message2);
    }
}
