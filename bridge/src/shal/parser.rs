use crate::shal::ast::{
    Action, Condition, Declaration, Input, Output, Program, SourceLoc, Statement,
};
use crate::shal::common::{Edge, IsWas, Value};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "shal/shal.pest"]
struct ShalParser;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Failed to parse input")]
    PestParseError(#[from] pest::error::Error<Rule>),
}

pub(crate) fn parse(input: &str) -> Result<Program, ParseError> {
    let pest_program = ShalParser::parse(Rule::program, input)?.next();

    let mut program = Program {
        declarations: vec![],
        statements: vec![],
    };

    let pest_program = pest_program.unwrap_or_else(|| unreachable!());

    for pair in pest_program.into_inner() {
        match pair.as_rule() {
            Rule::entity_declaration => {
                program.declarations.push(handle_entity_declaration(pair));
            }
            Rule::top_level_statement => {
                program.statements.push(handle_statement(pair));
            }
            _ => {}
        }
    }

    Ok(program)
}

fn handle_entity_declaration(pair: Pair<Rule>) -> Declaration {
    let declaration = pair.into_inner().next().unwrap();
    match declaration.as_rule() {
        Rule::input_declaration => handle_input_declaration(declaration),
        Rule::output_declaration => handle_output_declaration(declaration),
        _ => {
            unimplemented!()
        }
    }
}

fn handle_input_declaration(pair: Pair<Rule>) -> Declaration {
    let (line, col) = pair.line_col();
    let mut pairs = pair.into_inner();
    let entity_id = handle_entity_id(pairs.next().unwrap());
    let input = handle_input(pairs.next().unwrap());
    Declaration::Input {
        entity_id,
        number: input,
        source_loc: Some(SourceLoc(line, col)),
    }
}

fn handle_output_declaration(pair: Pair<Rule>) -> Declaration {
    let (line, col) = pair.line_col();
    let mut pairs = pair.into_inner();
    let entity_id = handle_entity_id(pairs.next().unwrap());
    let input = handle_output(pairs.next().unwrap());
    Declaration::Output {
        entity_id,
        number: input,
        source_loc: Some(SourceLoc(line, col)),
    }
}

fn handle_statement(pair: Pair<Rule>) -> Statement {
    let statement = pair.into_inner().next().unwrap();
    match statement.as_rule() {
        Rule::action => handle_action(statement),
        Rule::condition_block => handle_condition_block(statement),
        Rule::event_block => handle_event_block(statement),
        _ => {
            unimplemented!()
        }
    }
}

fn handle_action(pair: Pair<Rule>) -> Statement {
    let action = pair.into_inner().next().unwrap();
    Statement::Action(match action.as_rule() {
        Rule::toggle_action => handle_toggle_action(action),
        Rule::set_action => handle_set_action(action),
        _ => {
            unimplemented!()
        }
    })
}

fn handle_toggle_action(pair: Pair<Rule>) -> Action {
    Action::Toggle(handle_output_or_entity_id(
        pair.into_inner().next().unwrap(),
    ))
}

fn handle_input_or_entity_id(pair: Pair<Rule>) -> Input {
    match pair.as_rule() {
        Rule::input => Input::Number(handle_input(pair)),
        Rule::entity_id => Input::Entity(handle_entity_id(pair)),
        _ => {
            unimplemented!()
        }
    }
}

fn handle_output_or_entity_id(pair: Pair<Rule>) -> Output {
    match pair.as_rule() {
        Rule::output => Output::Number(handle_output(pair)),
        Rule::entity_id => Output::Entity(handle_entity_id(pair)),
        _ => {
            unimplemented!()
        }
    }
}

fn handle_input(pair: Pair<Rule>) -> u8 {
    handle_number(pair.into_inner().next().unwrap())
}

fn handle_output(pair: Pair<Rule>) -> u8 {
    handle_number(pair.into_inner().next().unwrap())
}

fn handle_number(pair: Pair<Rule>) -> u8 {
    if pair.as_rule() == Rule::number {
        pair.as_str().parse().unwrap()
    } else {
        unimplemented!()
    }
}

fn handle_entity_id(pair: Pair<Rule>) -> String {
    pair.as_str().to_string()
}

fn handle_value(pair: Pair<Rule>) -> Value {
    match pair.as_str() {
        "low" => Value::Low,
        "high" => Value::High,
        _ => unimplemented!(),
    }
}

fn handle_set_action(pair: Pair<Rule>) -> Action {
    let mut pairs = pair.into_inner();
    let output = handle_output_or_entity_id(pairs.next().unwrap());
    let value = handle_value(pairs.next().unwrap());
    Action::Set(output, value)
}

fn handle_condition_block(pair: Pair<Rule>) -> Statement {
    let mut pairs = pair.into_inner();
    let (condition, if_statements) = handle_if_block(pairs.next().unwrap());
    let else_statements = if let Some(else_block) = pairs.next() {
        handle_else_block(else_block)
    } else {
        vec![]
    };
    Statement::IfElse(condition, if_statements, else_statements)
}

fn handle_if_block(pair: Pair<Rule>) -> (Condition, Vec<Statement>) {
    let mut pairs = pair.into_inner();
    let condition = handle_condition(pairs.next().unwrap());
    let statements = pairs.map(|pair| handle_statement(pair)).collect();
    (condition, statements)
}

fn handle_else_block(pair: Pair<Rule>) -> Vec<Statement> {
    let mut pairs = pair.into_inner();
    if let Some(next) = pairs.next() {
        match next.as_rule() {
            Rule::condition_block => {
                vec![handle_condition_block(next)]
            }
            Rule::statement => {
                let mut result = vec![handle_statement(next)];
                for statement in pairs {
                    result.push(handle_statement(statement));
                }
                result
            }
            _ => unimplemented!(),
        }
    } else {
        vec![]
    }
}

fn handle_condition(pair: Pair<Rule>) -> Condition {
    let mut pairs = pair.into_inner();
    let lcondition = handle_lcondition(pairs.next().unwrap());
    if let Some(boolean_operator) = pairs.next() {
        let rcondition = handle_condition(pairs.next().unwrap());
        match boolean_operator.as_str() {
            "and" => Condition::And(Box::new(lcondition), Box::new(rcondition)),
            "or" => Condition::Or(Box::new(lcondition), Box::new(rcondition)),
            "xor" => Condition::Xor(Box::new(lcondition), Box::new(rcondition)),
            _ => unimplemented!(),
        }
    } else {
        lcondition
    }
}

fn handle_lcondition(pair: Pair<Rule>) -> Condition {
    let condition = pair.into_inner().next().unwrap();
    match condition.as_rule() {
        Rule::condition => handle_condition(condition),
        Rule::input_condition => handle_input_condition(condition),
        Rule::output_condition => handle_output_condition(condition),
        Rule::not_condition => handle_not_condition(condition),
        Rule::entity_condition => handle_entity_condition(condition),
        _ => unimplemented!(),
    }
}

fn handle_tspec(pair: Pair<Rule>) -> IsWas {
    match pair.as_str() {
        "is" => IsWas::Is,
        "was" => IsWas::Was,
        _ => unimplemented!(),
    }
}

fn handle_input_condition(pair: Pair<Rule>) -> Condition {
    let mut pairs = pair.into_inner();
    let input = Input::Number(handle_input(pairs.next().unwrap()));
    let tspec = handle_tspec(pairs.next().unwrap());
    let value = handle_value(pairs.next().unwrap());
    Condition::Input(input, tspec, value)
}

fn handle_output_condition(pair: Pair<Rule>) -> Condition {
    let mut pairs = pair.into_inner();
    let output = Output::Number(handle_output(pairs.next().unwrap()));
    let tspec = handle_tspec(pairs.next().unwrap());
    let value = handle_value(pairs.next().unwrap());
    Condition::Output(output, tspec, value)
}

fn handle_not_condition(pair: Pair<Rule>) -> Condition {
    Condition::Not(Box::new(handle_lcondition(
        pair.into_inner().next().unwrap(),
    )))
}

fn handle_entity_condition(pair: Pair<Rule>) -> Condition {
    let mut pairs = pair.into_inner();
    let entity = handle_entity_id(pairs.next().unwrap());
    let tspec = handle_tspec(pairs.next().unwrap());
    let value = handle_value(pairs.next().unwrap());
    Condition::Entity(entity, tspec, value)
}

fn handle_event_block(pair: Pair<Rule>) -> Statement {
    let mut pairs = pair.into_inner();
    let (edge, input) = handle_event(pairs.next().unwrap());
    let next = pairs.next();
    if let Some(next) = next {
        match next.as_rule() {
            Rule::action => Statement::Event {
                edge,
                input,
                statements: vec![handle_action(next)],
            },
            Rule::statement => {
                let mut statements = vec![];
                statements.push(handle_statement(next));
                for statement in pairs {
                    match statement.as_rule() {
                        Rule::statement => {
                            statements.push(handle_statement(statement));
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
        }
    } else {
        Statement::Event {
            edge,
            input,
            statements: vec![],
        }
    }
}

fn handle_event(pair: Pair<Rule>) -> (Edge, Input) {
    let mut pairs = pair.into_inner();
    let edge = handle_edge(pairs.next().unwrap());
    let input = handle_input_or_entity_id(pairs.next().unwrap());
    (edge, input)
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
        Action, Condition, Declaration, Input, Output, Program, SourceLoc, Statement,
    };
    use crate::shal::common;
    use crate::shal::common::{IsWas, Value};
    use crate::shal::parser::parse;

    #[test]
    fn test_parse() {
        assert_eq!(
            &parse("entity button = input 12;"),
            &Ok(Program {
                declarations: vec![Declaration::Input {
                    entity_id: "button".to_string(),
                    number: 12,
                    source_loc: Some(SourceLoc(1, 1)),
                }],
                statements: vec![],
            })
        );
        assert_eq!(
            &parse("entity light = output 5;"),
            &Ok(Program {
                declarations: vec![Declaration::Output {
                    entity_id: "light".to_string(),
                    number: 5,
                    source_loc: Some(SourceLoc(1, 1)),
                }],
                statements: vec![],
            })
        );
        assert_eq!(
            &parse("toggle output 1;"),
            &Ok(Program {
                declarations: vec![],
                statements: vec![Statement::Action(Action::Toggle(Output::Number(1)),)],
            })
        );
        assert_eq!(
            &parse("toggle light_downstairs;"),
            &Ok(Program {
                declarations: vec![],
                statements: vec![Statement::Action(Action::Toggle(Output::Entity(
                    "light_downstairs".to_string()
                )))],
            })
        );
        assert_eq!(
            &parse("set output 3 high;"),
            &Ok(Program {
                declarations: vec![],
                statements: vec![Statement::Action(Action::Set(
                    Output::Number(3),
                    Value::High
                ))],
            })
        );
        assert_eq!(
            &parse("set light_upstairs low;"),
            &Ok(Program {
                declarations: vec![],
                statements: vec![Statement::Action(Action::Set(
                    Output::Entity("light_upstairs".to_string()),
                    Value::Low
                ))],
            })
        );
        assert_eq!(
            &parse("on redge input 3 toggle output 4;"),
            &Ok(Program {
                declarations: vec![],
                statements: vec![Statement::Event {
                    edge: common::Edge::Rising,
                    input: Input::Number(3),
                    statements: vec![Statement::Action(Action::Toggle(Output::Number(4))),],
                },]
            })
        );
        assert_eq!(
            &parse("on fedge input 5 { toggle output 4; set output 6 high; }"),
            &Ok(Program {
                declarations: vec![],
                statements: vec![Statement::Event {
                    edge: common::Edge::Falling,
                    input: Input::Number(5),
                    statements: vec![
                        Statement::Action(Action::Toggle(Output::Number(4))),
                        Statement::Action(Action::Set(Output::Number(6), Value::High,)),
                    ],
                },]
            })
        );
        assert_eq!(
            &parse("if input 4 is low xor light_upstairs was high {} else { toggle output 4; }"),
            &Ok(Program {
                declarations: vec![],
                statements: vec![Statement::IfElse(
                    Condition::Xor(
                        Box::new(Condition::Input(Input::Number(4), IsWas::Is, Value::Low)),
                        Box::new(Condition::Entity(
                            "light_upstairs".to_string(),
                            IsWas::Was,
                            Value::High
                        )),
                    ),
                    vec![],
                    vec![Statement::Action(Action::Toggle(Output::Number(4))),],
                )],
            })
        );
        assert_eq!(
            &parse("if output 5 is high or output 20 is high {}"),
            &Ok(Program {
                declarations: vec![],
                statements: vec![Statement::IfElse(
                    Condition::Or(
                        Box::new(Condition::Output(Output::Number(5), IsWas::Is, Value::High)),
                        Box::new(Condition::Output(
                            Output::Number(20),
                            IsWas::Is,
                            Value::High
                        )),
                    ),
                    vec![],
                    vec![],
                )],
            })
        );
        let parse_result = parse(include_str!("../../static/standaertha.shal"));
        assert!(matches!(&parse_result, &Ok(Program { .. })));
    }
}
