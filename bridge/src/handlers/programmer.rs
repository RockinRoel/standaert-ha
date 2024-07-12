use crate::controller::message::MessageBody::{ProgramEndAck, ProgramStart, ProgramStartAck};
use crate::controller::message::{MessageBody, MAX_MESSAGE_BODY_LENGTH};
use crate::controller::program_header::PROGRAM_HEADER_LENGTH;
use crate::handlers::message::Message;
use crate::handlers::message::Message::{ReceivedFromController, SendToController};
use crate::handlers::programmer::HandleMessageResult::{Continue, Done};
use crate::handlers::programmer::State::{AwaitingAck, Uploading};
use crate::shal::bytecode::Program;
use crate::shal::{bytecode, compiler, parser};
use log::error;
use std::io;
use thiserror::Error;
use tokio::select;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::error::RecvError::{Closed, Lagged};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio_graceful_shutdown::SubsystemHandle;

#[derive(Copy, Clone)]
enum State {
    AwaitingAck,
    Uploading,
}

#[derive(Debug, Error)]
pub enum ProgrammerError {
    #[error("Error reading program")]
    IOError(#[from] io::Error),
    #[error("Error parsing program")]
    ParseError(#[from] parser::ParseError),
    #[error("Error compiling program")]
    CompileError(#[from] compiler::CompileError),
    #[error("Stack limit error")]
    StackLimitError(#[from] bytecode::StackLimitError),
    #[error("Program size error")]
    ProgramSizeError(#[from] bytecode::ProgramSizeError),
}

struct Programmer {
    subsys: SubsystemHandle,
    tx: Sender<Message>,
    rx: Receiver<Message>,
    state: State,
    program: Program,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum HandleMessageResult {
    Done,
    Continue,
}

pub async fn compile(program_path: &str) -> Result<Program, ProgrammerError> {
    let program_str = tokio::fs::read_to_string(&program_path).await?;
    let program_ast = parser::parse(&program_str)?;
    let program = compiler::compile(&program_ast)?;
    let _stack_depth = program.check_stack_depth(Some(32))?;
    let _program_size = program.check_program_size(Some(248))?;

    Ok(program)
}

pub async fn run(
    subsys: SubsystemHandle,
    program: Program,
    sender: Sender<Message>,
) -> Result<(), anyhow::Error> {
    sender
        .send(SendToController(ProgramStart {
            header: program.header(),
        }))
        .unwrap_or_else(|_| unreachable!());

    let rx = sender.subscribe();
    let mut programmer = Programmer {
        subsys,
        tx: sender,
        rx,
        state: AwaitingAck,
        program,
    };

    programmer.run().await;
    Ok(())
}

impl Programmer {
    async fn run(&mut self) {
        loop {
            select! {
                _ = self.subsys.on_shutdown_requested() => break,
                message = self.rx.recv() => if self.handle_message(&message) == Done {
                    break;
                },
            }
        }
    }

    fn handle_message(&mut self, message: &Result<Message, RecvError>) -> HandleMessageResult {
        match message {
            Ok(ReceivedFromController(body)) => match (self.state, body) {
                (AwaitingAck, ProgramStartAck { header }) => {
                    if *header == self.program.header() {
                        self.upload();
                    } else {
                        self.retry();
                    }
                }
                (Uploading, ProgramEndAck { header }) => {
                    if *header == self.program.header() {
                        return Done;
                    } else {
                        self.retry();
                    }
                }
                (_, _) => {}
            },
            Ok(_) => {}
            Err(Lagged(num_messages)) => {
                error!("Programmer lagging behind {num_messages} messages!");
            }
            Err(Closed) => return Done,
        }
        Continue
    }

    fn upload(&mut self) {
        let buf: Vec<u8> = (&self.program).into();
        let chunks: Vec<&[u8]> = buf[PROGRAM_HEADER_LENGTH..]
            .chunks(MAX_MESSAGE_BODY_LENGTH)
            .collect();
        let num_chunks = chunks.len();
        for chunk in chunks.iter().take(num_chunks - 1) {
            self.tx
                .send(SendToController(MessageBody::ProgramData {
                    code: (*chunk).into(),
                }))
                .unwrap_or_else(|_| unreachable!());
        }
        let last_chunk = *chunks.last().unwrap_or(&&[][..]);
        self.tx
            .send(SendToController(MessageBody::ProgramEnd {
                code: last_chunk.into(),
            }))
            .unwrap_or_else(|_| unreachable!());
        self.state = Uploading;
    }

    fn retry(&mut self) {
        self.tx
            .send(SendToController(ProgramStart {
                header: self.program.header(),
            }))
            .unwrap_or_else(|_| unreachable!());
        self.state = AwaitingAck;
    }
}
