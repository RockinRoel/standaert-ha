use crate::controller::program_header::ProgramHeader;
use crate::shal::ast::SourceLoc;
use crate::shal::common::{Edge, IsWas, Value};
use crc::{Crc, CRC_16_XMODEM};
use static_assertions::const_assert_eq;
use thiserror::Error;

const SET_VALUE_MASK: u8 = 0b0000_0001;
const ON_EDGE_MASK: u8 = 0b0000_0001;
const IF_IS_WAS_MASK: u8 = 0b0000_0100;
const IF_IO_MASK: u8 = 0b0000_0010;
const IF_VALUE_MASK: u8 = 0b0000_0001;

const SINGLE_BYTE_MASK: u8 = 0b0111_1111;
const DUAL_BYTE_MASK: u8 = 0b0011_1111;

const SINGLE_BYTE_PREFIX: u8 = 0b0000_0000;
const FIRST_BYTE_PREFIX: u8 = 0b1000_0000;
const SECOND_BYTE_PREFIX: u8 = 0b1100_0000;

const INSTR_END: u8 = 0b0000_0000;
const INSTR_AND: u8 = 0b0000_0001;
const INSTR_OR: u8 = 0b0000_0010;
const INSTR_XOR: u8 = 0b0000_0011;
const INSTR_NOT: u8 = 0b0000_0100;
const INSTR_POP: u8 = 0b0000_0101;

const INSTR_SET: u8 = 0b0000_0000;
const INSTR_TOGGLE: u8 = 0b0000_0010;
const INSTR_ON: u8 = 0b0000_0100;
const INSTR_IF: u8 = 0b0000_1000;

const INSTR_SET_MASK: u8 = DUAL_BYTE_MASK & !SET_VALUE_MASK;
const INSTR_TOGGLE_MASK: u8 = DUAL_BYTE_MASK;
const INSTR_ON_MASK: u8 = DUAL_BYTE_MASK & !ON_EDGE_MASK;
const INSTR_IF_MASK: u8 = DUAL_BYTE_MASK & !IF_IS_WAS_MASK & !IF_IO_MASK & !IF_VALUE_MASK;

const_assert_eq!(INSTR_SET_MASK, 0b0011_1110);
const_assert_eq!(INSTR_TOGGLE_MASK, 0b0011_1111);
const_assert_eq!(INSTR_ON_MASK, 0b0011_1110);
const_assert_eq!(INSTR_IF_MASK, 0b0011_1000);

pub(super) trait AsBit {
    fn as_bit(&self) -> bool;
    fn from_bit(b: bool) -> Self;
}

impl AsBit for Edge {
    fn as_bit(&self) -> bool {
        match self {
            Edge::Falling => false,
            Edge::Rising => true,
        }
    }

    fn from_bit(b: bool) -> Self {
        if b {
            Edge::Rising
        } else {
            Edge::Falling
        }
    }
}

impl AsBit for Value {
    fn as_bit(&self) -> bool {
        match self {
            Value::Low => false,
            Value::High => true,
        }
    }

    fn from_bit(b: bool) -> Self {
        if b {
            Value::High
        } else {
            Value::Low
        }
    }
}

impl AsBit for IsWas {
    fn as_bit(&self) -> bool {
        match self {
            IsWas::Was => false,
            IsWas::Is => true,
        }
    }

    fn from_bit(b: bool) -> Self {
        if b {
            IsWas::Is
        } else {
            IsWas::Was
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum InOut {
    Input,
    Output,
}

impl AsBit for InOut {
    fn as_bit(&self) -> bool {
        match self {
            InOut::Input => false,
            InOut::Output => true,
        }
    }

    fn from_bit(b: bool) -> Self {
        if b {
            InOut::Output
        } else {
            InOut::Input
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InstructionEncoding {
    SingleByte(u8),
    DualByte(u8, u8),
}

impl InstructionEncoding {
    fn single_byte(instruction: u8) -> InstructionEncoding {
        InstructionEncoding::SingleByte(SINGLE_BYTE_PREFIX | (instruction & SINGLE_BYTE_MASK))
    }

    fn dual_byte(instruction: u8, in_out: u8) -> InstructionEncoding {
        InstructionEncoding::DualByte(
            FIRST_BYTE_PREFIX | (instruction & DUAL_BYTE_MASK),
            SECOND_BYTE_PREFIX | (in_out & DUAL_BYTE_MASK),
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum Instruction {
    End,
    Pop,
    And,
    Or,
    Xor,
    Not,
    Set {
        output: u8,
        value: Value,
    },
    Toggle {
        output: u8,
    },
    On {
        input: u8,
        edge: Edge,
    },
    If {
        number: u8,
        is_was: IsWas,
        value: Value,
        in_out: InOut,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DecodingError {}

impl Instruction {
    fn byte_size(&self) -> usize {
        match *self {
            Instruction::End
            | Instruction::And
            | Instruction::Or
            | Instruction::Xor
            | Instruction::Not
            | Instruction::Pop => 1,
            Instruction::Set { .. }
            | Instruction::Toggle { .. }
            | Instruction::On { .. }
            | Instruction::If { .. } => 2,
        }
    }

    fn encode(&self) -> InstructionEncoding {
        match *self {
            Instruction::End => InstructionEncoding::single_byte(INSTR_END),
            Instruction::And => InstructionEncoding::single_byte(INSTR_AND),
            Instruction::Or => InstructionEncoding::single_byte(INSTR_OR),
            Instruction::Xor => InstructionEncoding::single_byte(INSTR_XOR),
            Instruction::Not => InstructionEncoding::single_byte(INSTR_NOT),
            Instruction::Pop => InstructionEncoding::single_byte(INSTR_POP),
            Instruction::Set { output, value } => {
                let mut instr = INSTR_SET;
                if value.as_bit() {
                    instr |= SET_VALUE_MASK;
                }
                InstructionEncoding::dual_byte(instr, output)
            }
            Instruction::Toggle { output } => InstructionEncoding::dual_byte(INSTR_TOGGLE, output),
            Instruction::On { input, edge } => {
                let mut instr = INSTR_ON;
                if edge.as_bit() {
                    instr |= ON_EDGE_MASK;
                }
                InstructionEncoding::dual_byte(instr, input)
            }
            Instruction::If {
                number,
                is_was,
                value,
                in_out,
            } => {
                const INSTR: u8 = INSTR_IF;
                let mut instr = INSTR;
                if is_was.as_bit() {
                    instr |= IF_IS_WAS_MASK;
                }
                if in_out.as_bit() {
                    instr |= IF_IO_MASK;
                }
                if value.as_bit() {
                    instr |= IF_VALUE_MASK;
                }
                InstructionEncoding::dual_byte(instr, number)
            }
        }
    }

    fn decode(encoding: &InstructionEncoding) -> Result<Instruction, DecodingError> {
        match *encoding {
            InstructionEncoding::SingleByte(instr) => match instr & SINGLE_BYTE_MASK {
                INSTR_END => Ok(Instruction::End),
                INSTR_AND => Ok(Instruction::And),
                INSTR_OR => Ok(Instruction::Or),
                INSTR_XOR => Ok(Instruction::Xor),
                INSTR_NOT => Ok(Instruction::Not),
                INSTR_POP => Ok(Instruction::Pop),
                _ => Err(DecodingError {}),
            },
            InstructionEncoding::DualByte(instr, value) => {
                let value = value & DUAL_BYTE_MASK;
                if instr & INSTR_SET_MASK == INSTR_SET {
                    Ok(Instruction::Set {
                        output: value,
                        value: Value::from_bit(instr & SET_VALUE_MASK != 0),
                    })
                } else if instr & INSTR_TOGGLE_MASK == INSTR_TOGGLE {
                    Ok(Instruction::Toggle { output: value })
                } else if instr & INSTR_ON_MASK == INSTR_ON {
                    Ok(Instruction::On {
                        input: value,
                        edge: Edge::from_bit(instr & ON_EDGE_MASK != 0),
                    })
                } else if instr & INSTR_IF_MASK == INSTR_IF {
                    Ok(Instruction::If {
                        number: value,
                        is_was: IsWas::from_bit(instr & IF_IS_WAS_MASK != 0),
                        value: Value::from_bit(instr & IF_VALUE_MASK != 0),
                        in_out: InOut::from_bit(instr & IF_IO_MASK != 0),
                    })
                } else {
                    Err(DecodingError {})
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub(super) instructions: Vec<Instruction>,
    pub(super) source_locations: Vec<SourceLoc>,
}

#[derive(Copy, Clone, Error, Debug, Eq, PartialEq)]
#[error("Stack limit error")]
pub struct StackLimitError {
    source_location: Option<SourceLoc>,
}

#[derive(Copy, Clone, Error, Debug, Eq, PartialEq)]
#[error("Program size error")]
pub struct ProgramSizeError {
    source_location: Option<SourceLoc>,
}

impl Program {
    pub(crate) fn check_stack_depth(&self, limit: Option<i32>) -> Result<i32, StackLimitError> {
        let mut depth = 0;
        let mut max = 0;
        for (i, instr) in self.instructions.iter().enumerate() {
            match *instr {
                Instruction::Pop | Instruction::And | Instruction::Or | Instruction::Xor => {
                    depth -= 1;
                }
                Instruction::On { .. } | Instruction::If { .. } => {
                    depth += 1;
                    max = std::cmp::max(depth, max);
                    if let Some(limit) = limit {
                        if max > limit {
                            return Err(StackLimitError {
                                source_location: self.source_locations.get(i).copied(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(max)
    }

    pub(crate) fn check_program_size(
        &self,
        limit: Option<usize>,
    ) -> Result<usize, ProgramSizeError> {
        let mut length = 0;
        for (i, instr) in self.instructions.iter().enumerate() {
            length += instr.byte_size();
            if let Some(limit) = limit {
                if length > limit {
                    return Err(ProgramSizeError {
                        source_location: self.source_locations.get(i).copied(),
                    });
                }
            }
        }
        Ok(length)
    }

    pub(crate) fn calc_length(&self) -> usize {
        self.check_program_size(None)
            .unwrap_or_else(|_| unreachable!())
    }

    pub(crate) fn calc_crc(&self) -> u16 {
        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let mut digest = crc.digest();
        for instr in &self.instructions {
            let encoding = instr.encode();
            match encoding {
                InstructionEncoding::SingleByte(b) => digest.update(&[b]),
                InstructionEncoding::DualByte(b1, b2) => digest.update(&[b1, b2]),
            }
        }
        digest.finalize()
    }

    pub(crate) fn header(&self) -> ProgramHeader {
        ProgramHeader {
            length: self.calc_length() as u16, // TODO(Roel): ???
            crc: self.calc_crc(),
        }
    }
}

impl TryFrom<&[u8]> for Program {
    type Error = DecodingError;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err(DecodingError {});
        }
        if &bytes[0..4] != b"SHAL" {
            return Err(DecodingError {});
        }
        let read_length = u16::from_be_bytes([bytes[4], bytes[5]]);
        let read_crc = u16::from_be_bytes([bytes[6], bytes[7]]);
        if bytes.len() - (read_length as usize) != 8 {
            return Err(DecodingError {});
        }
        let program_code = &bytes[8..];
        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let mut digest = crc.digest();
        digest.update(program_code);
        let crc = digest.finalize();
        if crc != read_crc {
            return Err(DecodingError {});
        }
        let mut instructions = vec![];
        let mut first_byte: Option<u8> = None;
        for b in program_code {
            if let Some(fb) = first_byte {
                if *b & SECOND_BYTE_PREFIX == 0 {
                    return Err(DecodingError {});
                }
                instructions.push(Instruction::decode(&InstructionEncoding::DualByte(fb, *b))?);
                first_byte = None;
            } else if *b & !DUAL_BYTE_MASK == FIRST_BYTE_PREFIX {
                first_byte = Some(*b);
            } else if *b & !DUAL_BYTE_MASK == SECOND_BYTE_PREFIX {
                return Err(DecodingError {});
            } else {
                instructions.push(Instruction::decode(&InstructionEncoding::SingleByte(*b))?);
            }
        }
        Ok(Program {
            instructions,
            source_locations: vec![],
        })
    }
}

impl From<&Program> for Vec<u8> {
    fn from(program: &Program) -> Self {
        let mut bytecode = vec![];
        for instr in program.instructions.iter() {
            match instr.encode() {
                InstructionEncoding::SingleByte(b) => {
                    bytecode.push(b);
                }
                InstructionEncoding::DualByte(b1, b2) => {
                    bytecode.push(b1);
                    bytecode.push(b2);
                }
            }
        }

        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let mut digest = crc.digest();
        digest.update(&bytecode);
        let crc = digest.finalize().to_be_bytes();

        let length = (bytecode.len() as u16).to_be_bytes();

        let mut header: Vec<u8> = vec![];
        header.extend_from_slice(b"SHAL");
        header.extend_from_slice(&length);
        header.extend_from_slice(&crc);

        let mut result = header;
        result.append(&mut bytecode);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::shal::bytecode::Instruction::{End, If, Not, On, Or, Pop, Set, Toggle};
    use crate::shal::bytecode::{
        Edge, InOut, IsWas, Program, ProgramSizeError, StackLimitError, Value,
    };

    #[test]
    fn test_program() {
        let program = Program {
            instructions: vec![
                On {
                    input: 0,
                    edge: Edge::Rising,
                },
                Toggle { output: 0 },
                Pop,
                On {
                    input: 1,
                    edge: Edge::Rising,
                },
                Toggle { output: 1 },
                Pop,
                If {
                    number: 0,
                    value: Value::High,
                    is_was: IsWas::Is,
                    in_out: InOut::Output,
                },
                If {
                    number: 1,
                    value: Value::High,
                    is_was: IsWas::Is,
                    in_out: InOut::Output,
                },
                Or,
                Set {
                    output: 2,
                    value: Value::High,
                },
                Not,
                Set {
                    output: 2,
                    value: Value::Low,
                },
                Pop,
                End,
            ],
            source_locations: vec![],
        };
        assert_eq!(Ok(2), program.check_stack_depth(None));
        assert_eq!(
            Err(StackLimitError {
                source_location: None
            }),
            program.check_stack_depth(Some(1))
        );
        assert_eq!(Ok(22), program.check_program_size(None));
        assert_eq!(
            Err(ProgramSizeError {
                source_location: None
            }),
            program.check_program_size(Some(21))
        );
        let encoded: Vec<u8> = (&program).into();
        assert_eq!(
            &[
                b'S',
                b'H',
                b'A',
                b'L',
                0,
                22,
                0x59,
                0x78,
                0b1000_0101, // on rising edge
                0b1100_0000, // input 0
                0b1000_0010, // toggle
                0b1100_0000, // output 0
                0b0000_0101, // pop
                0b1000_0101, // on rising edge
                0b1100_0001, // input 1
                0b1000_0010, // toggle
                0b1100_0001, // output 1
                0b0000_0101, // pop
                0b1000_1111, // if output is high
                0b1100_0000, // output 0
                0b1000_1111, // if output is high
                0b1100_0001, // output 1
                0b0000_0010, // or
                0b1000_0001, // set high
                0b1100_0010, // output 2
                0b0000_0100, // not
                0b1000_0000, // set low
                0b1100_0010, // output 2
                0b0000_0101, // pop
                0b0000_0000  // end
            ],
            &encoded[..]
        );
        let decoded = Program::try_from(&encoded[..]);
        assert_eq!(Ok(&program), decoded.as_ref());
    }
}
