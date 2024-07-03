use crate::shal::bytecode::Instruction;
use crate::shal::common;
use crate::shal::compiler::CompileError::{DuplicateEntityError, UnknownEntityError};
use crate::shal::compiler::InOut::{Input, Output};
use crate::shal::{ast, bytecode};
use std::collections::HashMap;
use std::fmt::Display;
use thiserror::Error;

enum InOut {
    Input(u8),
    Output(u8),
}

#[derive(Error, Debug, PartialEq)]
pub enum CompileError {
    #[error("Duplicate entity")]
    DuplicateEntityError {
        name: String,
        first_loc: Option<ast::SourceLoc>,
        second_loc: Option<ast::SourceLoc>,
    },
    #[error("Unknown entity")]
    UnknownEntityError {
        name: String,
        location: Option<ast::SourceLoc>,
    },
}

fn retrieve_input(
    entities: &HashMap<String, InOut>,
    input: &ast::Input,
) -> Result<u8, CompileError> {
    match input {
        ast::Input::Number(number) => Ok(*number),
        ast::Input::Entity(entity_id) => {
            if let Some(Input(number)) = entities.get(entity_id) {
                Ok(*number)
            } else {
                Err(UnknownEntityError {
                    name: entity_id.clone(),
                    location: None,
                })
            }
        }
    }
}

fn retrieve_output(
    entities: &HashMap<String, InOut>,
    output: &ast::Output,
) -> Result<u8, CompileError> {
    match output {
        ast::Output::Number(number) => Ok(*number),
        ast::Output::Entity(entity_id) => {
            if let Some(Output(number)) = entities.get(entity_id) {
                Ok(*number)
            } else {
                Err(UnknownEntityError {
                    name: entity_id.clone(),
                    location: None,
                })
            }
        }
    }
}

fn retrieve_entity(
    entities: &HashMap<String, InOut>,
    entity: &str,
) -> Result<(u8, bytecode::InOut), CompileError> {
    match entities.get(entity) {
        Some(Input(number)) => Ok((*number, bytecode::InOut::Input)),
        Some(Output(number)) => Ok((*number, bytecode::InOut::Output)),
        _ => Err(UnknownEntityError {
            name: entity.to_string(),
            location: None,
        }),
    }
}

fn collect_entities(
    declarations: &[ast::Declaration],
) -> Result<HashMap<String, InOut>, CompileError> {
    let mut result = HashMap::new();
    for declaration in declarations.iter() {
        match declaration {
            ast::Declaration::Input { entity_id, number } => {
                if result.contains_key(entity_id) {
                    return Err(DuplicateEntityError {
                        name: entity_id.clone(),
                        first_loc: None,
                        second_loc: None,
                    });
                } else {
                    result.insert(entity_id.clone(), Input(*number));
                }
            }
            ast::Declaration::Output { entity_id, number } => {
                if result.contains_key(entity_id) {
                    return Err(DuplicateEntityError {
                        name: entity_id.clone(),
                        first_loc: None,
                        second_loc: None,
                    });
                } else {
                    result.insert(entity_id.clone(), Output(*number));
                }
            }
        }
    }
    Ok(result)
}

pub(crate) fn compile(ast_program: &ast::Program) -> Result<bytecode::Program, CompileError> {
    let entities = collect_entities(&ast_program.declarations)?;
    let mut bytecode_program = bytecode::Program {
        instructions: vec![],
        source_locations: vec![],
    };
    for statement in ast_program.statements.iter() {
        handle_statement(&mut bytecode_program, &entities, statement);
    }
    bytecode_program.instructions.push(Instruction::End);
    Ok(bytecode_program)
}

fn handle_statement(
    program: &mut bytecode::Program,
    entities: &HashMap<String, InOut>,
    statement: &ast::Statement,
) {
    match statement {
        ast::Statement::Action(action) => handle_action(program, entities, action),
        ast::Statement::IfElse(condition, if_block, else_block) => {
            handle_if_else(program, entities, condition, if_block, else_block);
        }
        ast::Statement::Event {
            edge,
            input,
            statements,
        } => {
            handle_event(program, entities, edge, input, statements);
        }
    }
}

fn handle_action(
    program: &mut bytecode::Program,
    entities: &HashMap<String, InOut>,
    action: &ast::Action,
) {
    match action {
        ast::Action::Toggle(output) => {
            let number = retrieve_output(entities, output).unwrap();
            program
                .instructions
                .push(Instruction::Toggle { output: number });
        }
        ast::Action::Set(output, value) => {
            let number = retrieve_output(entities, output).unwrap();
            program.instructions.push(Instruction::Set {
                output: number,
                value: *value,
            });
        }
    }
}

fn handle_if_else(
    program: &mut bytecode::Program,
    entities: &HashMap<String, InOut>,
    condition: &ast::Condition,
    if_block: &[ast::Statement],
    else_block: &[ast::Statement],
) {
    handle_condition(program, entities, condition);
    for statement in if_block.iter() {
        handle_statement(program, entities, statement);
    }
    if !else_block.is_empty() {
        program.instructions.push(Instruction::Not);
        for statement in else_block.iter() {
            handle_statement(program, entities, statement);
        }
    }
    program.instructions.push(Instruction::Pop);
}

fn handle_condition(
    program: &mut bytecode::Program,
    entities: &HashMap<String, InOut>,
    condition: &ast::Condition,
) {
    match condition {
        ast::Condition::And(l, r) => {
            handle_condition(program, entities, l.as_ref());
            handle_condition(program, entities, r.as_ref());
            program.instructions.push(Instruction::And);
        }
        ast::Condition::Or(l, r) => {
            handle_condition(program, entities, l.as_ref());
            handle_condition(program, entities, r.as_ref());
            program.instructions.push(Instruction::Or);
        }
        ast::Condition::Xor(l, r) => {
            handle_condition(program, entities, l.as_ref());
            handle_condition(program, entities, r.as_ref());
            program.instructions.push(Instruction::Xor);
        }
        ast::Condition::Not(c) => {
            handle_condition(program, entities, c.as_ref());
            program.instructions.push(Instruction::Not);
        }
        ast::Condition::Input(input, is_was, value) => {
            let number = retrieve_input(entities, input).unwrap();
            program.instructions.push(Instruction::If {
                number,
                is_was: *is_was,
                value: *value,
                in_out: bytecode::InOut::Input,
            });
        }
        ast::Condition::Output(output, is_was, value) => {
            let number = retrieve_output(entities, output).unwrap();
            program.instructions.push(Instruction::If {
                number,
                is_was: *is_was,
                value: *value,
                in_out: bytecode::InOut::Output,
            });
        }
        ast::Condition::Entity(entity, is_was, value) => {
            let (number, in_out) = retrieve_entity(entities, entity).unwrap();
            program.instructions.push(Instruction::If {
                number,
                is_was: *is_was,
                value: *value,
                in_out,
            });
        }
    }
}

fn handle_event(
    program: &mut bytecode::Program,
    entities: &HashMap<String, InOut>,
    edge: &common::Edge,
    input: &ast::Input,
    statements: &[ast::Statement],
) {
    let number = retrieve_input(entities, input).unwrap();
    program.instructions.push(Instruction::On {
        input: number,
        edge: *edge,
    });
    for statement in statements.iter() {
        handle_statement(program, entities, statement);
    }
    program.instructions.push(Instruction::Pop);
}

#[cfg(test)]
mod tests {
    use crate::shal::ast;
    use crate::shal::bytecode;
    use crate::shal::bytecode::Instruction;
    use crate::shal::common::{Edge, IsWas, Value};
    use crate::shal::compiler::compile;

    #[test]
    fn test_compile() {
        let ast_program = ast::Program {
            declarations: vec![
                ast::Declaration::Input {
                    entity_id: "button_downstairs".to_string(),
                    number: 0,
                },
                ast::Declaration::Input {
                    entity_id: "button_upstairs".to_string(),
                    number: 1,
                },
                ast::Declaration::Output {
                    entity_id: "light_downstairs".to_string(),
                    number: 0,
                },
                ast::Declaration::Output {
                    entity_id: "light_upstairs".to_string(),
                    number: 1,
                },
                ast::Declaration::Output {
                    entity_id: "light_stairs".to_string(),
                    number: 2,
                },
            ],
            statements: vec![
                ast::Statement::Event {
                    edge: Edge::Rising,
                    input: ast::Input::Entity("button_downstairs".to_string()),
                    statements: vec![ast::Statement::Action(ast::Action::Toggle(
                        ast::Output::Entity("light_downstairs".to_string()),
                    ))],
                },
                ast::Statement::Event {
                    edge: Edge::Rising,
                    input: ast::Input::Entity("button_upstairs".to_string()),
                    statements: vec![ast::Statement::Action(ast::Action::Toggle(
                        ast::Output::Entity("light_upstairs".to_string()),
                    ))],
                },
                ast::Statement::IfElse(
                    ast::Condition::Or(
                        Box::new(ast::Condition::Output(
                            ast::Output::Entity("light_downstairs".to_string()),
                            IsWas::Is,
                            Value::High,
                        )),
                        Box::new(ast::Condition::Output(
                            ast::Output::Entity("light_upstairs".to_string()),
                            IsWas::Is,
                            Value::High,
                        )),
                    ),
                    vec![ast::Statement::Action(ast::Action::Set(
                        ast::Output::Entity("light_stairs".to_string()),
                        Value::High,
                    ))],
                    vec![ast::Statement::Action(ast::Action::Set(
                        ast::Output::Entity("light_stairs".to_string()),
                        Value::Low,
                    ))],
                ),
            ],
        };
        let bytecode_program = compile(&ast_program);

        assert_eq!(
            &Ok(bytecode::Program {
                instructions: vec![
                    Instruction::On {
                        input: 0,
                        edge: Edge::Rising,
                    },
                    Instruction::Toggle { output: 0 },
                    Instruction::Pop,
                    Instruction::On {
                        input: 1,
                        edge: Edge::Rising,
                    },
                    Instruction::Toggle { output: 1 },
                    Instruction::Pop,
                    Instruction::If {
                        number: 0,
                        value: Value::High,
                        is_was: IsWas::Is,
                        in_out: bytecode::InOut::Output,
                    },
                    Instruction::If {
                        number: 1,
                        value: Value::High,
                        is_was: IsWas::Is,
                        in_out: bytecode::InOut::Output,
                    },
                    Instruction::Or,
                    Instruction::Set {
                        output: 2,
                        value: Value::High,
                    },
                    Instruction::Not,
                    Instruction::Set {
                        output: 2,
                        value: Value::Low,
                    },
                    Instruction::Pop,
                    Instruction::End,
                ],
                source_locations: vec![],
            }),
            &bytecode_program
        );
    }
}
