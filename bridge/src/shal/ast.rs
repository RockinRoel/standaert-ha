use crate::shal::common::{Edge, IsWas, Value};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use regex::RegexBuilder;
use thiserror::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SourceLoc(pub usize, pub usize);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(try_from = "String")]
pub struct EntityID {
    id: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("Invalid entity ID: {id}")]
pub struct InvalidEntityIDError {
    id: String,
}

impl Display for EntityID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.id, f)
    }
}

impl TryFrom<String> for EntityID {
    type Error = InvalidEntityIDError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let entity_id_regex = RegexBuilder::new("^[A-Za-z][A-Za-z0-9_]*$").unicode(false).build().unwrap_or_else(|_| unreachable!());
        if entity_id_regex.is_match(&value) {
            Ok(EntityID { id: value })
        } else {
            Err(InvalidEntityIDError { id: value })
        }
    }
}

impl<'a> TryFrom<&'a str> for EntityID {
    type Error = InvalidEntityIDError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.to_owned().try_into()
    }
}

impl From<EntityID> for String {
    fn from(value: EntityID) -> Self {
        value.id
    }
}

impl<'a> From<&'a EntityID> for &'a str {
    fn from(value: &'a EntityID) -> Self {
        &value.id
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(try_from = "u8")]
pub struct PinID {
    id: u8,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("Invalid pin ID: {id}, ids must be in range [0, 32)")]
pub struct InvalidPinIDError {
    id: u8,
}

impl Display for PinID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.id, f)
    }
}

impl TryFrom<u8> for PinID {
    type Error = InvalidPinIDError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 32 {
            Ok(PinID { id: value })
        } else {
            Err(InvalidPinIDError { id: value })
        }
    }
}

impl From<PinID> for u8 {
    fn from(value: PinID) -> Self {
        value.id
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct IODeclarations {
    #[serde(with = "::serde_with::rust::maps_duplicate_key_is_error")]
    pub inputs: HashMap<EntityID, IODeclaration>,
    #[serde(with = "::serde_with::rust::maps_duplicate_key_is_error")]
    pub outputs: HashMap<EntityID, IODeclaration>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IODeclaration {
    pub pin: PinID,
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
    Entity(EntityID, IsWas, Value),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Input {
    Number(PinID),
    Entity(EntityID),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Output {
    Number(PinID),
    Entity(EntityID),
}
