use crate::shal::common::{Edge, IsWas, Value};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SourceLoc(pub usize, pub usize);

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct IODeclarations {
    #[serde(default)]
    pub inputs: HashMap<String, IODeclaration>,
    #[serde(default)]
    pub outputs: HashMap<String, IODeclaration>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct IODeclaration {
    pub pin: u8,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Program {
    pub(super) declarations: IODeclarations,
    pub(super) statements: Vec<Statement>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeclarationType {
    Input,
    Output,
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
