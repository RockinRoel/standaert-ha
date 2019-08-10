pub mod config;
#[cfg(feature = "mqtt")]
pub mod mqtt;
#[cfg(feature = "webthing")]
pub mod webthing;

use std::error::Error;
use std::fmt;

pub const SLIP_END: u8 = 0o300;
pub const SLIP_ESC: u8 = 0o333;
pub const SLIP_ESC_END: u8 = 0o334;
pub const SLIP_ESC_ESC: u8 = 0o335;

#[derive(Debug, Clone)]
pub struct SlipError;

impl fmt::Display for SlipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SLIP decoding error")
    }
}

impl Error for SlipError {
    fn description(&self) -> &str {
        "SLIP decoding error"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct CRCError;

impl fmt::Display for CRCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CRC error")
    }
}

impl Error for CRCError {
    fn description(&self) -> &str {
        "CRC error"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

pub fn verify_crc16(buf: &[u8]) -> bool {
    use crc16::*;
    let len = buf.len();
    let crc_from_buf = (u16::from(buf[len - 2]) << 8) | u16::from(buf[len - 1]);
    let crc_calc = State::<XMODEM>::calculate(&buf[0..len - 2]);
    crc_calc == crc_from_buf
}

pub fn append_crc16(mut buf: Vec<u8>) -> Vec<u8> {
    use crc16::*;
    let crc = State::<XMODEM>::calculate(&buf);
    buf.push(((crc >> 8) & 0xFF) as u8);
    buf.push((crc & 0xFF) as u8);
    buf
}

pub fn slip_encode(buf: &[u8]) -> Vec<u8> {
    let mut result = vec![];
    result.push(SLIP_END);
    for byte in buf.iter() {
        match *byte {
            SLIP_END => {
                result.push(SLIP_ESC);
                result.push(SLIP_ESC_END);
            }
            SLIP_ESC => {
                result.push(SLIP_ESC);
                result.push(SLIP_ESC_ESC);
            }
            b => {
                result.push(b);
            }
        }
    }
    result.push(SLIP_END);
    result
}

pub fn slip_decode(buf: &[u8]) -> Result<Vec<u8>, SlipError> {
    let mut result = vec![];
    let len = buf.len();
    if len < 2 || buf[0] != SLIP_END || buf[len - 1] != SLIP_END {
        return Err(SlipError {});
    }
    let mut esc = false;
    for byte in buf[1..len - 1].iter() {
        if esc {
            match *byte {
                SLIP_ESC_END => result.push(SLIP_END),
                SLIP_ESC_ESC => result.push(SLIP_ESC),
                _ => return Err(SlipError),
            }
            esc = false;
        } else {
            match *byte {
                SLIP_ESC => esc = true,
                SLIP_END => return Err(SlipError),
                b => result.push(b),
            }
        }
    }
    Ok(result)
}

#[derive(Copy, Clone)]
enum PackageInputStreamMode {
    Scan,
    Read,
}

pub struct PackageInputStream<I>
where
    I: Iterator<Item = Result<u8, std::io::Error>>,
{
    input: I,
    mode: PackageInputStreamMode,
    buf: Vec<u8>,
}

impl<I> PackageInputStream<I>
where
    I: Iterator<Item = Result<u8, std::io::Error>>,
{
    pub fn new(input: I) -> PackageInputStream<I> {
        PackageInputStream {
            input,
            mode: PackageInputStreamMode::Scan,
            buf: vec![],
        }
    }
}

impl<I> Iterator for PackageInputStream<I>
where
    I: Iterator<Item = Result<u8, std::io::Error>>,
{
    type Item = Result<Vec<u8>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        for b in &mut self.input {
            match b {
                Ok(b) => match (b, self.mode) {
                    (SLIP_END, PackageInputStreamMode::Scan) => {
                        self.buf.push(b);
                        self.mode = PackageInputStreamMode::Read;
                    }
                    (SLIP_END, PackageInputStreamMode::Read) => {
                        self.buf.push(b);
                        let result = slip_decode(&self.buf);
                        match result {
                            Err(e) => {
                                return Some(Err(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    e,
                                )));
                            }
                            Ok(mut result) => {
                                self.buf.clear();
                                self.mode = PackageInputStreamMode::Scan;
                                if verify_crc16(&result) {
                                    result.pop();
                                    result.pop();
                                    return Some(Ok(result));
                                } else {
                                    return Some(Err(std::io::Error::new(
                                        std::io::ErrorKind::Other,
                                        CRCError,
                                    )));
                                }
                            }
                        }
                    }
                    (_b, PackageInputStreamMode::Read) => {
                        self.buf.push(b);
                    }
                    (_, PackageInputStreamMode::Scan) => {}
                },
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::TimedOut {
                        return Some(Err(e));
                    }
                }
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    PressStart,
    PressEnd,
}

impl EventType {
    fn value(self) -> u8 {
        use EventType::*;
        match self {
            PressStart => 0x00,
            PressEnd => 0x80,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Event {
    data: u8,
}

impl Event {
    pub fn new(event_type: EventType, button: u8) -> Event {
        Event {
            data: event_type.value() | 0x40 | (button & 0x1F),
        }
    }

    pub fn frow_raw(data: u8) -> Event {
        Event { data }
    }

    pub fn raw(self) -> u8 {
        self.data
    }

    pub fn valid(self) -> bool {
        self.data & 0x40 != 0
    }

    pub fn event_type(self) -> EventType {
        use EventType::*;
        if self.data & 0x80 == 0 {
            PressStart
        } else {
            PressEnd
        }
    }

    pub fn button(self) -> u8 {
        self.data & 0x1F
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event( ")?;
        if self.valid() {
            write!(f, "{:?}( button: {}, ", self.event_type(), self.button())?;
        } else {
            write!(f, "Invalid( ")?;
        }
        write!(f, "raw: 0x{:02x} ) )", self.raw())
    }
}

#[derive(Copy, Clone)]
pub enum CommandType {
    None,
    Refresh,
    Toggle,
    Off,
    On,
}

impl CommandType {
    fn value(self) -> u8 {
        use CommandType::*;
        match self {
            None => 0x00,
            Refresh => 0x20,
            Toggle => 0x40,
            Off => 0x80,
            On => 0xC0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Command {
    data: u8,
}

impl Command {
    pub fn new(command_type: CommandType, output: u8) -> Command {
        Command {
            data: command_type.value() | (output & 0x1F),
        }
    }

    pub fn from_raw(data: u8) -> Command {
        Command { data }
    }

    pub fn raw(self) -> u8 {
        self.data
    }

    pub fn command_type(self) -> CommandType {
        use CommandType::*;
        match self.data & 0xE0 {
            0x00 => None,
            0x20 => Refresh,
            0x40 => Toggle,
            0x80 => Off,
            0xC0 => On,
            _ => panic!("Unknown command type"),
        }
    }

    pub fn output(self) -> u8 {
        self.data & 0x1F
    }
}

pub struct Package {
    events: Vec<Event>,
    state: u32,
}

impl Package {
    pub fn from_buf(buf: &[u8]) -> Package {
        if buf.len() != 36 {
            panic!("Buf length is not 36!");
        }
        Package {
            events: (&buf[0..32]).iter().map(|b| Event::frow_raw(*b)).collect(),
            state: (u32::from(buf[32]) << 24)
                | (u32::from(buf[33]) << 16)
                | (u32::from(buf[34]) << 8)
                | u32::from(buf[35]),
        }
    }
}

impl fmt::Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Package( state: {:032b}, events: [ ", self.state)?;
        let mut first = true;
        for event in self.events.iter() {
            if event.valid() {
                if first {
                    first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", event)?;
            }
        }
        write!(f, " ] )")
    }
}

pub trait Service {
    fn handle_package(&mut self, package: &Package);
    fn join(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let buf = &[SLIP_END, SLIP_ESC, 0x12, 0x33];
        let res = slip_encode(buf);
        assert_eq!(
            res,
            &[
                SLIP_END,
                SLIP_ESC,
                SLIP_ESC_END,
                SLIP_ESC,
                SLIP_ESC_ESC,
                0x12,
                0x33,
                SLIP_END
            ]
        );
    }

    #[test]
    fn test_decode() {
        let buf = &[
            SLIP_END,
            SLIP_ESC,
            SLIP_ESC_END,
            SLIP_ESC,
            SLIP_ESC_ESC,
            0x12,
            0x33,
            SLIP_END,
        ];
        let res = slip_decode(buf).unwrap();
        assert_eq!(res, &[SLIP_END, SLIP_ESC, 0x12, 0x33]);
    }

    #[test]
    fn test_crc() {
        let buf = vec![1, 2, 3, 4, 5];
        let buf = append_crc16(buf);
        assert!(verify_crc16(&buf));
    }

    #[test]
    fn test_iter() {
        let buf = &[
            1, 2, 3, SLIP_END, 12, 0xC1, 0x8C, SLIP_END, 7, 9, SLIP_END, 88, 19, 0xA5, 0x44,
            SLIP_END,
        ];
        let mut stream = PackageInputStream::new(buf.into_iter().map(|b| Ok(*b)));
        let pkg1 = stream.next().unwrap().unwrap();
        assert_eq!(pkg1, vec![12]);
        let pkg2 = stream.next().unwrap().unwrap();
        assert_eq!(pkg2, vec![88, 19]);
        let pkg3 = stream.next();
        assert!(pkg3.is_none());
    }
}
