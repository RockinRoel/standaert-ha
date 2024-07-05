use crate::shal::common::{Edge, IsWas, Value};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SourceLoc {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Program {
    pub(super) declarations: Vec<Declaration>,
    pub(super) statements: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Declaration {
    Input { entity_id: String, number: u8 },
    Output { entity_id: String, number: u8 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Statement {
    Action(Action),
    IfElse(Condition, Vec<Statement>, Vec<Statement>),
    Event {
        edge: Edge,
        input: Input,
        statements: Vec<Statement>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Action {
    Toggle(Output),
    Set(Output, Value),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Condition {
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Xor(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
    Input(Input, IsWas, Value),
    Output(Output, IsWas, Value),
    Entity(String, IsWas, Value),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Input {
    Number(u8),
    Entity(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Output {
    Number(u8),
    Entity(String),
}
