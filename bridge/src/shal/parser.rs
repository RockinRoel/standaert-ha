use crate::shal::ast::{
    Action, Condition, EntityID, IODeclarations, Input, InvalidEntityIDError, InvalidPinIDError,
    Output, PinID, Program, Statement,
};
use crate::shal::common::{Edge, IsWas, Value};
use crate::shal::parser::ParseError::{
    DoubleInputPinError, DoubleOutputPinError, DuplicateEntityIDError,
};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use regex::RegexBuilder;
use std::collections::HashSet;
use std::hash::Hash;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "shal/shal.pest"]
struct ShalParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse entity declarations")]
    EntityParseError(#[from] deser_hjson::Error),
    #[error("Failed to parse program")]
    PestParseError(#[from] Box<pest::error::Error<Rule>>),
    #[error("Duplicate entity id: {id}, all entity ids must be unique")]
    DuplicateEntityIDError { id: EntityID },
    #[error("Double use of input pin {pin} for two different inputs")]
    DoubleInputPinError { pin: PinID },
    #[error("Double use of output pin {pin} for two different outputs")]
    DoubleOutputPinError { pin: PinID },
    #[error("Invalid pin ID")]
    InvalidPinIDError(#[from] InvalidPinIDError),
    #[error("Invalid entity ID")]
    InvalidEntityIDError(#[from] InvalidEntityIDError),
}

pub(crate) fn parse(input: &str) -> Result<Program, ParseError> {
    let mut program = Program::default();

    let separator = RegexBuilder::new(r"^---\s*$")
        .multi_line(true)
        .build()
        .unwrap_or_else(|_| unreachable!());
    let splits: Vec<&str> = separator.splitn(input, 2).collect();
    if splits.len() == 2 {
        let first_split = *splits.first().unwrap_or_else(|| unreachable!());
        let declarations = deser_hjson::from_str(first_split)?;
        validate_declarations(&declarations)?;
        program.declarations = declarations;
    }

    if splits.is_empty() {
        return Ok(program);
    }

    let last_split = *splits.last().unwrap_or_else(|| unreachable!());
    let pest_program = ShalParser::parse(Rule::program, last_split)
        .map_err(Box::new)?
        .next();

    let pest_program = pest_program.unwrap_or_else(|| unreachable!());

    for pair in pest_program.into_inner() {
        if Rule::top_level_statement == pair.as_rule() {
            program.statements.push(handle_statement(pair)?);
        }
    }

    Ok(program)
}

fn validate_declarations(declarations: &IODeclarations) -> Result<(), ParseError> {
    let inputs = &declarations.inputs;
    let outputs = &declarations.outputs;
    if let Some(id) = find_double(inputs.keys().chain(outputs.keys())) {
        return Err(DuplicateEntityIDError { id: id.clone() });
    }
    if let Some(pin) = find_double(inputs.values().map(|d| d.pin)) {
        return Err(DoubleInputPinError { pin });
    }
    if let Some(pin) = find_double(outputs.values().map(|d| d.pin)) {
        return Err(DoubleOutputPinError { pin });
    }
    Ok(())
}

fn find_double<Item: Eq + Hash, I: Iterator<Item = Item>>(iterator: I) -> Option<Item> {
    let mut set = HashSet::new();
    for i in iterator {
        if set.contains(&i) {
            return Some(i);
        }
        set.insert(i);
    }
    None
}

fn handle_statement(pair: Pair<Rule>) -> Result<Statement, ParseError> {
    let statement = pair.into_inner().next().unwrap();
    Ok(match statement.as_rule() {
        Rule::action => handle_action(statement)?,
        Rule::condition_block => handle_condition_block(statement)?,
        Rule::event_block => handle_event_block(statement)?,
        _ => {
            unimplemented!()
        }
    })
}

fn handle_action(pair: Pair<Rule>) -> Result<Statement, ParseError> {
    let action = pair.into_inner().next().unwrap();
    Ok(Statement::Action(match action.as_rule() {
        Rule::toggle_action => handle_toggle_action(action)?,
        Rule::set_action => handle_set_action(action)?,
        _ => {
            unimplemented!()
        }
    }))
}

fn handle_toggle_action(pair: Pair<Rule>) -> Result<Action, ParseError> {
    Ok(Action::Toggle(handle_output_or_entity_id(
        pair.into_inner().next().unwrap(),
    )?))
}

fn handle_input_or_entity_id(pair: Pair<Rule>) -> Result<Input, ParseError> {
    Ok(match pair.as_rule() {
        Rule::input => Input::Number(handle_input(pair)?),
        Rule::entity_id => Input::Entity(handle_entity_id(pair)?),
        _ => {
            unimplemented!()
        }
    })
}

fn handle_output_or_entity_id(pair: Pair<Rule>) -> Result<Output, ParseError> {
    Ok(match pair.as_rule() {
        Rule::output => Output::Number(handle_output(pair)?),
        Rule::entity_id => Output::Entity(handle_entity_id(pair)?),
        _ => {
            unimplemented!()
        }
    })
}

fn handle_input(pair: Pair<Rule>) -> Result<PinID, ParseError> {
    handle_number(pair.into_inner().next().unwrap())
}

fn handle_output(pair: Pair<Rule>) -> Result<PinID, ParseError> {
    handle_number(pair.into_inner().next().unwrap())
}

fn handle_number(pair: Pair<Rule>) -> Result<PinID, ParseError> {
    if pair.as_rule() == Rule::pin_id {
        Ok(pair.as_str().parse::<u8>().unwrap().try_into()?)
    } else {
        unimplemented!()
    }
}

fn handle_entity_id(pair: Pair<Rule>) -> Result<EntityID, ParseError> {
    Ok(pair.as_str().to_owned().try_into()?)
}

fn handle_value(pair: Pair<Rule>) -> Value {
    match pair.as_str() {
        "low" => Value::Low,
        "high" => Value::High,
        _ => unimplemented!(),
    }
}

fn handle_set_action(pair: Pair<Rule>) -> Result<Action, ParseError> {
    let mut pairs = pair.into_inner();
    let output = handle_output_or_entity_id(pairs.next().unwrap())?;
    let value = handle_value(pairs.next().unwrap());
    Ok(Action::Set(output, value))
}

fn handle_condition_block(pair: Pair<Rule>) -> Result<Statement, ParseError> {
    let mut pairs = pair.into_inner();
    let (condition, if_statements) = handle_if_block(pairs.next().unwrap())?;
    let else_statements = if let Some(else_block) = pairs.next() {
        handle_else_block(else_block)?
    } else {
        vec![]
    };
    Ok(Statement::IfElse(condition, if_statements, else_statements))
}

fn handle_if_block(pair: Pair<Rule>) -> Result<(Condition, Vec<Statement>), ParseError> {
    let mut pairs = pair.into_inner();
    let condition = handle_condition(pairs.next().unwrap())?;
    let statements: Result<Vec<_>, _> = pairs.map(|pair| handle_statement(pair)).collect();
    Ok((condition, statements?))
}

fn handle_else_block(pair: Pair<Rule>) -> Result<Vec<Statement>, ParseError> {
    let mut pairs = pair.into_inner();
    if let Some(next) = pairs.next() {
        Ok(match next.as_rule() {
            Rule::condition_block => {
                vec![handle_condition_block(next)?]
            }
            Rule::statement => {
                let mut result = vec![handle_statement(next)?];
                for statement in pairs {
                    result.push(handle_statement(statement)?);
                }
                result
            }
            _ => unimplemented!(),
        })
    } else {
        Ok(vec![])
    }
}

fn handle_condition(pair: Pair<Rule>) -> Result<Condition, ParseError> {
    let mut pairs = pair.into_inner();
    let lcondition = handle_lcondition(pairs.next().unwrap())?;
    if let Some(boolean_operator) = pairs.next() {
        let rcondition = handle_condition(pairs.next().unwrap())?;
        Ok(match boolean_operator.as_str() {
            "and" => Condition::And(Box::new(lcondition), Box::new(rcondition)),
            "or" => Condition::Or(Box::new(lcondition), Box::new(rcondition)),
            "xor" => Condition::Xor(Box::new(lcondition), Box::new(rcondition)),
            _ => unimplemented!(),
        })
    } else {
        Ok(lcondition)
    }
}

fn handle_lcondition(pair: Pair<Rule>) -> Result<Condition, ParseError> {
    let condition = pair.into_inner().next().unwrap();
    Ok(match condition.as_rule() {
        Rule::condition => handle_condition(condition)?,
        Rule::input_condition => handle_input_condition(condition)?,
        Rule::output_condition => handle_output_condition(condition)?,
        Rule::not_condition => handle_not_condition(condition)?,
        Rule::entity_condition => handle_entity_condition(condition)?,
        _ => unimplemented!(),
    })
}

fn handle_tspec(pair: Pair<Rule>) -> IsWas {
    match pair.as_str() {
        "is" => IsWas::Is,
        "was" => IsWas::Was,
        _ => unimplemented!(),
    }
}

fn handle_input_condition(pair: Pair<Rule>) -> Result<Condition, ParseError> {
    let mut pairs = pair.into_inner();
    let input = Input::Number(handle_input(pairs.next().unwrap())?);
    let tspec = handle_tspec(pairs.next().unwrap());
    let value = handle_value(pairs.next().unwrap());
    Ok(Condition::Input(input, tspec, value))
}

fn handle_output_condition(pair: Pair<Rule>) -> Result<Condition, ParseError> {
    let mut pairs = pair.into_inner();
    let output = Output::Number(handle_output(pairs.next().unwrap())?);
    let tspec = handle_tspec(pairs.next().unwrap());
    let value = handle_value(pairs.next().unwrap());
    Ok(Condition::Output(output, tspec, value))
}

fn handle_not_condition(pair: Pair<Rule>) -> Result<Condition, ParseError> {
    Ok(Condition::Not(Box::new(handle_lcondition(
        pair.into_inner().next().unwrap(),
    )?)))
}

fn handle_entity_condition(pair: Pair<Rule>) -> Result<Condition, ParseError> {
    let mut pairs = pair.into_inner();
    let entity = handle_entity_id(pairs.next().unwrap())?;
    let tspec = handle_tspec(pairs.next().unwrap());
    let value = handle_value(pairs.next().unwrap());
    Ok(Condition::Entity(entity, tspec, value))
}

fn handle_event_block(pair: Pair<Rule>) -> Result<Statement, ParseError> {
    let mut pairs = pair.into_inner();
    let (edge, input) = handle_event(pairs.next().unwrap())?;
    let next = pairs.next();
    if let Some(next) = next {
        Ok(match next.as_rule() {
            Rule::action => Statement::Event {
                edge,
                input,
                statements: vec![handle_action(next)?],
            },
            Rule::statement => {
                let mut statements = vec![];
                statements.push(handle_statement(next)?);
                for statement in pairs {
                    match statement.as_rule() {
                        Rule::statement => {
                            statements.push(handle_statement(statement)?);
                        }
                        _ => unimplemented!(),
                    }
                }
                Statement::Event {
                    edge,
                    input,
                    statements,
                }
            }
            _ => unimplemented!(),
        })
    } else {
        Ok(Statement::Event {
            edge,
            input,
            statements: vec![],
        })
    }
}

fn handle_event(pair: Pair<Rule>) -> Result<(Edge, Input), ParseError> {
    let mut pairs = pair.into_inner();
    let edge = handle_edge(pairs.next().unwrap());
    let input = handle_input_or_entity_id(pairs.next().unwrap())?;
    Ok((edge, input))
}

fn handle_edge(pair: Pair<Rule>) -> Edge {
    match pair.as_str() {
        "redge" => Edge::Rising,
        "fedge" => Edge::Falling,
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::shal::ast::{
        Action, Condition, IODeclaration, IODeclarations, Input, Output, Program, Statement,
    };
    use crate::shal::common;
    use crate::shal::common::{IsWas, Value};
    use crate::shal::parser::parse;
    use std::collections::HashMap;

    #[test]
    fn test_parse() {
        assert_eq!(
            &parse("{inputs: {button: {pin: 12}}}\n---\n").unwrap(),
            &Program {
                declarations: IODeclarations {
                    inputs: HashMap::from([(
                        "button".try_into().unwrap(),
                        IODeclaration {
                            pin: 12.try_into().unwrap(),
                            name: None
                        }
                    ),]),
                    outputs: Default::default(),
                },
                statements: vec![],
            }
        );
        assert_eq!(
            &parse("{outputs: {light: {pin: 12}}}\n---\n").unwrap(),
            &Program {
                declarations: IODeclarations {
                    inputs: Default::default(),
                    outputs: HashMap::from([(
                        "light".try_into().unwrap(),
                        IODeclaration {
                            pin: 12.try_into().unwrap(),
                            name: None
                        }
                    ),]),
                },
                statements: vec![],
            }
        );
        assert_eq!(
            &parse("toggle output 1;").unwrap(),
            &Program {
                declarations: Default::default(),
                statements: vec![Statement::Action(Action::Toggle(Output::Number(
                    1.try_into().unwrap()
                )),)],
            }
        );
        assert_eq!(
            &parse("toggle light_downstairs;").unwrap(),
            &Program {
                declarations: Default::default(),
                statements: vec![Statement::Action(Action::Toggle(Output::Entity(
                    "light_downstairs".try_into().unwrap()
                )))],
            }
        );
        assert_eq!(
            &parse("set output 3 high;").unwrap(),
            &Program {
                declarations: Default::default(),
                statements: vec![Statement::Action(Action::Set(
                    Output::Number(3.try_into().unwrap()),
                    Value::High
                ))],
            }
        );
        assert_eq!(
            &parse("set light_upstairs low;").unwrap(),
            &Program {
                declarations: Default::default(),
                statements: vec![Statement::Action(Action::Set(
                    Output::Entity("light_upstairs".try_into().unwrap()),
                    Value::Low
                ))],
            }
        );
        assert_eq!(
            &parse("on redge input 3 toggle output 4;").unwrap(),
            &Program {
                declarations: Default::default(),
                statements: vec![Statement::Event {
                    edge: common::Edge::Rising,
                    input: Input::Number(3.try_into().unwrap()),
                    statements: vec![Statement::Action(Action::Toggle(Output::Number(
                        4.try_into().unwrap()
                    ))),],
                },]
            }
        );
        assert_eq!(
            &parse("on fedge input 5 { toggle output 4; set output 6 high; }").unwrap(),
            &Program {
                declarations: Default::default(),
                statements: vec![Statement::Event {
                    edge: common::Edge::Falling,
                    input: Input::Number(5.try_into().unwrap()),
                    statements: vec![
                        Statement::Action(Action::Toggle(Output::Number(4.try_into().unwrap()))),
                        Statement::Action(Action::Set(
                            Output::Number(6.try_into().unwrap()),
                            Value::High,
                        )),
                    ],
                },]
            }
        );
        assert_eq!(
            &parse("if input 4 is low xor light_upstairs was high {} else { toggle output 4; }")
                .unwrap(),
            &Program {
                declarations: Default::default(),
                statements: vec![Statement::IfElse(
                    Condition::Xor(
                        Box::new(Condition::Input(
                            Input::Number(4.try_into().unwrap()),
                            IsWas::Is,
                            Value::Low
                        )),
                        Box::new(Condition::Entity(
                            "light_upstairs".try_into().unwrap(),
                            IsWas::Was,
                            Value::High
                        )),
                    ),
                    vec![],
                    vec![Statement::Action(Action::Toggle(Output::Number(
                        4.try_into().unwrap()
                    ))),],
                )],
            }
        );
        assert_eq!(
            &parse("if output 5 is high or output 20 is high {}").unwrap(),
            &Program {
                declarations: Default::default(),
                statements: vec![Statement::IfElse(
                    Condition::Or(
                        Box::new(Condition::Output(
                            Output::Number(5.try_into().unwrap()),
                            IsWas::Is,
                            Value::High
                        )),
                        Box::new(Condition::Output(
                            Output::Number(20.try_into().unwrap()),
                            IsWas::Is,
                            Value::High
                        )),
                    ),
                    vec![],
                    vec![],
                )],
            }
        );
        let parse_result = parse(include_str!("../../static/standaertha.shal"));
        assert!(matches!(&parse_result, &Ok(Program { .. })));
    }
}
