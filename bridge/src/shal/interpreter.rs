use crate::shal::bytecode::{AsBit, InOut, Instruction, Program};
use crate::shal::common::{Edge, IsWas, Value};

struct VmState<'a> {
    input_old: &'a FixedBitSet,
    input_new: &'a FixedBitSet,
    output_old: &'a FixedBitSet,
    output_new: FixedBitSet,
    stack: BitStack,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FixedBitSet {
    set: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BitSetError {
    OutOfBounds,
}

impl FixedBitSet {
    fn new() -> Self {
        FixedBitSet { set: 0 }
    }

    fn set(&mut self, bit: u8, value: bool) -> Result<(), BitSetError> {
        if bit >= 32 {
            Err(BitSetError::OutOfBounds)
        } else if value {
            self.set |= 1 << bit;
            Ok(())
        } else {
            self.set &= !(1 << bit);
            Ok(())
        }
    }

    fn get(&self, bit: u8) -> Result<bool, BitSetError> {
        if bit >= 32 {
            Err(BitSetError::OutOfBounds)
        } else {
            Ok(self.set & (1 << bit) != 0)
        }
    }

    fn clear(&mut self) {
        self.set = 0;
    }

    fn value(&self) -> u32 {
        self.set
    }
}

impl From<u32> for FixedBitSet {
    fn from(value: u32) -> Self {
        Self { set: value }
    }
}

impl From<FixedBitSet> for u32 {
    fn from(bitset: FixedBitSet) -> Self {
        bitset.value()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct BitStack {
    stack: u32,
    stack_depth: u8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum StackError {
    StackOverflow,
    StackUnderflow,
}

impl BitStack {
    fn new() -> Self {
        BitStack {
            stack: 0,
            stack_depth: 0,
        }
    }

    fn push(&mut self, value: bool) -> Result<(), StackError> {
        if self.stack_depth >= 32 {
            Err(StackError::StackOverflow)
        } else {
            if value {
                self.stack |= 1 << self.stack_depth;
            } else {
                self.stack &= !(1 << self.stack_depth);
            }
            self.stack_depth += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<bool, StackError> {
        if self.stack_depth == 0 {
            Err(StackError::StackUnderflow)
        } else {
            let result = self.peek();
            self.stack_depth -= 1;
            Ok(result.unwrap())
        }
    }

    fn peek(&self) -> Option<bool> {
        if self.stack_depth == 0 {
            None
        } else {
            Some(self.stack & (1 << (self.stack_depth - 1)) != 0)
        }
    }

    fn all_one(&self) -> bool {
        let mask = 0xFFFF_FFFF >> (32 - self.stack_depth);
        self.stack & mask == mask
    }
}

fn run_program(
    program: &Program,
    input_old: &FixedBitSet,
    input_new: &FixedBitSet,
    output_old: &FixedBitSet,
) -> FixedBitSet {
    let mut state = VmState {
        input_old,
        input_new,
        output_old,
        output_new: *output_old,
        stack: BitStack::new(),
    };
    for instr in program.instructions.iter() {
        match instr {
            Instruction::End => {
                return state.output_new;
            }
            Instruction::Not => {
                let b = state.stack.pop().unwrap();
                state.stack.push(!b).unwrap();
            }
            Instruction::And => {
                let b1 = state.stack.pop().unwrap();
                let b2 = state.stack.pop().unwrap();
                state.stack.push(b1 && b2).unwrap();
            }
            Instruction::Or => {
                let b1 = state.stack.pop().unwrap();
                let b2 = state.stack.pop().unwrap();
                state.stack.push(b1 || b2).unwrap();
            }
            Instruction::Xor => {
                let b1 = state.stack.pop().unwrap();
                let b2 = state.stack.pop().unwrap();
                state.stack.push(b1 == b2).unwrap();
            }
            Instruction::Pop => {
                state.stack.pop().unwrap();
            }
            Instruction::If {
                number,
                is_was,
                value,
                in_out,
            } => {
                let bitset = match (is_was, in_out) {
                    (IsWas::Was, InOut::Input) => state.input_old,
                    (IsWas::Is, InOut::Input) => state.input_new,
                    (IsWas::Was, InOut::Output) => state.output_old,
                    (IsWas::Is, InOut::Output) => &state.output_new,
                };
                state
                    .stack
                    .push(&Value::from_bit(bitset.get((*number).into()).unwrap()) == value)
                    .unwrap();
            }
            Instruction::On { input, edge } => {
                let before = Value::from_bit(state.input_old.get((*input).into()).unwrap());
                let now = Value::from_bit(state.input_new.get((*input).into()).unwrap());
                state
                    .stack
                    .push(matches!(
                        (edge, before, now),
                        (Edge::Rising, Value::Low, Value::High)
                            | (Edge::Falling, Value::High, Value::Low)
                    ))
                    .unwrap();
            }
            Instruction::Toggle { output } if state.stack.all_one() => {
                let before = state.output_new.get((*output).into()).unwrap();
                state.output_new.set((*output).into(), !before).unwrap();
            }
            Instruction::Set { output, value } if state.stack.all_one() => {
                state
                    .output_new
                    .set((*output).into(), value.as_bit())
                    .unwrap();
            }
            _ => {}
        }
    }
    state.output_new
}

#[cfg(test)]
mod tests {
    use crate::shal::bytecode::Instruction::{End, If, Not, On, Or, Pop, Set, Toggle};
    use crate::shal::bytecode::{InOut, Program};
    use crate::shal::common::{Edge, IsWas, Value};
    use crate::shal::interpreter::{run_program, FixedBitSet};

    #[test]
    fn test_interpret() {
        let program = Program {
            declarations: Default::default(),
            instructions: vec![
                On {
                    input: 0.try_into().unwrap(),
                    edge: Edge::Rising,
                },
                Toggle {
                    output: 0.try_into().unwrap(),
                },
                Pop,
                On {
                    input: 1.try_into().unwrap(),
                    edge: Edge::Rising,
                },
                Toggle {
                    output: 1.try_into().unwrap(),
                },
                Pop,
                If {
                    number: 0.try_into().unwrap(),
                    value: Value::High,
                    is_was: IsWas::Is,
                    in_out: InOut::Output,
                },
                If {
                    number: 1.try_into().unwrap(),
                    value: Value::High,
                    is_was: IsWas::Is,
                    in_out: InOut::Output,
                },
                Or,
                Set {
                    output: 2.try_into().unwrap(),
                    value: Value::High,
                },
                Not,
                Set {
                    output: 2.try_into().unwrap(),
                    value: Value::Low,
                },
                Pop,
                End,
            ],
            source_locations: vec![],
        };
        assert_eq!(
            FixedBitSet::from(0x0000_0005),
            run_program(
                &program,
                &0x0000_0000.into(),
                &0x0000_0001.into(),
                &0x0000_0000.into()
            )
        );
        assert_eq!(
            FixedBitSet::from(0x0000_0006),
            run_program(
                &program,
                &0x0000_0000.into(),
                &0x0000_0002.into(),
                &0x0000_0000.into()
            )
        );

        assert_eq!(
            FixedBitSet::from(0x0000_0000),
            run_program(
                &program,
                &0x0000_0000.into(),
                &0x0000_0001.into(),
                &0x0000_0005.into()
            )
        );
        assert_eq!(
            FixedBitSet::from(0x0000_0000),
            run_program(
                &program,
                &0x0000_0000.into(),
                &0x0000_0002.into(),
                &0x0000_0006.into()
            )
        );

        assert_eq!(
            FixedBitSet::from(0x0000_0005),
            run_program(
                &program,
                &0x0000_0000.into(),
                &0x0000_0002.into(),
                &0x0000_0007.into()
            )
        );
        assert_eq!(
            FixedBitSet::from(0x0000_0006),
            run_program(
                &program,
                &0x0000_0000.into(),
                &0x0000_0001.into(),
                &0x0000_0007.into()
            )
        );

        assert_eq!(
            FixedBitSet::from(0x0000_0007),
            run_program(
                &program,
                &0x0000_0000.into(),
                &0x0000_0001.into(),
                &0x0000_0006.into()
            )
        );
        assert_eq!(
            FixedBitSet::from(0x0000_0007),
            run_program(
                &program,
                &0x0000_0000.into(),
                &0x0000_0002.into(),
                &0x0000_0005.into()
            )
        );
    }
}
