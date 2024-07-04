use crate::controller::message::MessageBody::{ProgramEndAck, ProgramStartAck};
use crate::controller::message::{MessageBody, MAX_MESSAGE_BODY_LENGTH};
use crate::controller::program_header::PROGRAM_HEADER_LENGTH;
use crate::handlers::handler::{HandleResult, Handler};
use crate::handlers::message::Message;
use crate::handlers::message::Message::ReceivedFromController;
use crate::handlers::programmer::State::Neutral;
use crate::shal::bytecode::Program;
use crate::shal::{compiler, parser};
use std::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

enum State {
    Neutral,
    AwaitingAck,
    Uploading,
}

pub struct Programmer {
    program_path: String,
    state: State,
    program: Option<Program>,
    sender: UnboundedSender<Message>,
}

impl Programmer {
    pub fn new(program_path: String, sender: UnboundedSender<Message>) -> Self {
        Programmer {
            program_path,
            state: Neutral,
            program: None,
            sender,
        }
    }
}

impl Programmer {
    fn reload_program(&mut self) {
        // TODO(Roel): handle errors!
        let program_str =
            std::fs::read_to_string(&self.program_path).expect("Failed to read program!");
        let program_ast = parser::parse(&program_str).expect("Failed to parse program!");
        let program_bytecode = compiler::compile(&program_ast).expect("Failed to compile program!");
        let _stack_depth = program_bytecode.check_stack_depth(Some(32)).expect("Stack too deep!");
        let _program_length = program_bytecode.check_program_length(Some(248)).expect("Program too long!");

        self.program = Some(program_bytecode);
        self.announce_program_start();
    }

    fn reset(&mut self) {
        self.state = Neutral;
        self.program = None;
    }

    fn announce_program_start(&mut self) {
        match &self.program {
            Some(program) => {
                self.sender
                    .send(Message::SendToController(MessageBody::ProgramStart {
                        header: program.header(),
                    }))
                    .expect("Could not send?"); // TODO(Roel): how to handle?
                self.state = State::AwaitingAck;
            }
            _ => {
                // Wrong state?
            }
        }
    }

    fn upload(&mut self) {
        match (&self.state, &self.program) {
            (State::AwaitingAck, Some(program)) => {
                let buf: Vec<u8> = program.into();
                let chunks: Vec<&[u8]> = buf[PROGRAM_HEADER_LENGTH..]
                    .chunks(MAX_MESSAGE_BODY_LENGTH)
                    .collect();
                let num_chunks = chunks.len();
                for i in 0..(num_chunks - 1) {
                    let chunk = chunks[i];
                    self.sender
                        .send(Message::SendToController(MessageBody::ProgramData {
                            code: chunk.into(),
                        }))
                        .expect("Could not send?"); // TODO(Roel): how to handle?
                }
                let last_chunk = *chunks.last().unwrap_or(&&[][..]);
                self.sender
                    .send(Message::SendToController(MessageBody::ProgramEnd {
                        code: last_chunk.into(),
                    }))
                    .expect("Could not send?"); // TODO(Roel): how to handle?
                self.state = State::Uploading;
            }
            _ => {
                panic!("Upload called when in wrong state, or when there was no program!");
            }
        }
    }
}

impl Handler for Programmer {
    fn handle(&mut self, message: &Message) -> HandleResult {
        match (&self.state, &self.program, message) {
            (
                State::AwaitingAck,
                Some(program),
                ReceivedFromController(ProgramStartAck { header }),
            ) => {
                if &program.header() == header {
                    self.upload();
                } else {
                    self.announce_program_start();
                }
            }
            (State::Uploading, Some(program), ReceivedFromController(ProgramEndAck { header })) => {
                if &program.header() == header {
                    self.reset();
                } else {
                    self.announce_program_start();
                }
            }
            (_, _, Message::ReloadProgram) => {
                self.reload_program();
            }
            _ => {}
        }
        HandleResult::Continue
    }
}
