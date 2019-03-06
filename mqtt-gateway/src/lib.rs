pub const SLIP_END: u8 = 0o300;
pub const SLIP_ESC: u8 = 0o333;
pub const SLIP_ESC_END: u8 = 0o334;
pub const SLIP_ESC_ESC: u8 = 0o335;

#[derive(Debug, Clone)]
pub struct SlipError;

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
                _ => return Err(SlipError {}),
            }
            esc = false;
        } else {
            match *byte {
                SLIP_ESC => esc = true,
                SLIP_END => return Err(SlipError {}),
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
                        if result.is_err() {
                            return Some(Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "SLIP decode error",
                            )));
                        }
                        let mut result = result.unwrap();
                        self.buf.clear();
                        self.mode = PackageInputStreamMode::Scan;
                        if verify_crc16(&result) {
                            result.pop();
                            result.pop();
                            return Some(Ok(result));
                        } else {
                            return Some(Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "CRC failure",
                            )));
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

pub enum EventType {
    PressStart,
    PressEnd,
}

#[derive(Copy, Clone)]
pub struct Event {
    data: u8,
}

impl Event {
    pub fn new(event_type: EventType, button: u8) -> Event {
        let event_mask = match event_type {
            EventType::PressStart => 0x00,
            EventType::PressEnd => 0x80,
        };
        Event {
            data: event_mask | 0x40 | (button & 0x1F),
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
        if self.data & 0x80 == 0 {
            EventType::PressStart
        } else {
            EventType::PressEnd
        }
    }

    pub fn button(self) -> u8 {
        self.data & 0x1F
    }
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
