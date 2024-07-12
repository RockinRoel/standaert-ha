use crate::shal::ast::{EntityID, IODeclaration, IODeclarations, PinID};
use crate::shal::bytecode::Instruction;
use crate::shal::common;
use crate::shal::compiler::CompileError::UnknownEntityError;
use crate::shal::{ast, bytecode};
use thiserror::Error;

fn loc_to_string(source_loc: &Option<ast::SourceLoc>) -> String {
    if let Some(ast::SourceLoc(line, col)) = source_loc {
        format!("line {}, col {}", line, col)
    } else {
        "<null>".to_string()
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CompileError {
    #[error("Unknown entity: {0} at {1}", name, loc_to_string(location))]
    UnknownEntityError {
        name: EntityID,
        location: Option<ast::SourceLoc>,
    },
}

fn retrieve_input(declarations: &IODeclarations, input: &ast::Input) -> Result<PinID, CompileError> {
    match input {
        ast::Input::Number(number) => Ok(*number),
        ast::Input::Entity(entity_id) => {
            if let Some(IODeclaration { pin, .. }) = declarations.inputs.get(entity_id) {
                Ok(*pin)
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
    declarations: &IODeclarations,
    output: &ast::Output,
) -> Result<PinID, CompileError> {
    match output {
        ast::Output::Number(number) => Ok(*number),
        ast::Output::Entity(entity_id) => {
            if let Some(IODeclaration { pin, .. }) = declarations.outputs.get(entity_id) {
                Ok(*pin)
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
    declarations: &IODeclarations,
    entity_id: &EntityID,
) -> Result<(PinID, bytecode::InOut), CompileError> {
    retrieve_input(declarations, &ast::Input::Entity(entity_id.clone()))
        .map(|i| (i, bytecode::InOut::Input))
        .or_else(|_| {
            retrieve_output(declarations, &ast::Output::Entity(entity_id.clone()))
                .map(|i| (i, bytecode::InOut::Output))
        })
}

pub(crate) fn compile(ast_program: &ast::Program) -> Result<bytecode::Program, CompileError> {
    let mut bytecode_program = bytecode::Program {
        declarations: ast_program.declarations.clone(),
        instructions: vec![],
        source_locations: vec![],
    };
    for statement in ast_program.statements.iter() {
        handle_statement(&mut bytecode_program, statement);
    }
    bytecode_program.instructions.push(Instruction::End);
    Ok(bytecode_program)
}

fn handle_statement(program: &mut bytecode::Program, statement: &ast::Statement) {
    match statement {
        ast::Statement::Action(action) => handle_action(program, action),
        ast::Statement::IfElse(condition, if_block, else_block) => {
            handle_if_else(program, condition, if_block, else_block);
        }
        ast::Statement::Event {
            edge,
            input,
            statements,
        } => {
            handle_event(program, edge, input, statements);
        }
    }
}

fn handle_action(program: &mut bytecode::Program, action: &ast::Action) {
    match action {
        ast::Action::Toggle(output) => {
            let number = retrieve_output(&program.declarations, output).unwrap();
            program
                .instructions
                .push(Instruction::Toggle { output: number });
        }
        ast::Action::Set(output, value) => {
            let number = retrieve_output(&program.declarations, output).unwrap();
            program.instructions.push(Instruction::Set {
                output: number,
                value: *value,
            });
        }
    }
}

fn handle_if_else(
    program: &mut bytecode::Program,
    condition: &ast::Condition,
    if_block: &[ast::Statement],
    else_block: &[ast::Statement],
) {
    handle_condition(program, condition);
    for statement in if_block.iter() {
        handle_statement(program, statement);
    }
    if !else_block.is_empty() {
        program.instructions.push(Instruction::Not);
        for statement in else_block.iter() {
            handle_statement(program, statement);
        }
    }
    program.instructions.push(Instruction::Pop);
}

fn handle_condition(program: &mut bytecode::Program, condition: &ast::Condition) {
    match condition {
        ast::Condition::And(l, r) => {
            handle_condition(program, l.as_ref());
            handle_condition(program, r.as_ref());
            program.instructions.push(Instruction::And);
        }
        ast::Condition::Or(l, r) => {
            handle_condition(program, l.as_ref());
            handle_condition(program, r.as_ref());
            program.instructions.push(Instruction::Or);
        }
        ast::Condition::Xor(l, r) => {
            handle_condition(program, l.as_ref());
            handle_condition(program, r.as_ref());
            program.instructions.push(Instruction::Xor);
        }
        ast::Condition::Not(c) => {
            handle_condition(program, c.as_ref());
            program.instructions.push(Instruction::Not);
        }
        ast::Condition::Input(input, is_was, value) => {
            let number = retrieve_input(&program.declarations, input).unwrap();
            program.instructions.push(Instruction::If {
                number,
                is_was: *is_was,
                value: *value,
                in_out: bytecode::InOut::Input,
            });
        }
        ast::Condition::Output(output, is_was, value) => {
            let number = retrieve_output(&program.declarations, output).unwrap();
            program.instructions.push(Instruction::If {
                number,
                is_was: *is_was,
                value: *value,
                in_out: bytecode::InOut::Output,
            });
        }
        ast::Condition::Entity(entity, is_was, value) => {
            let (number, in_out) = retrieve_entity(&program.declarations, entity).unwrap();
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
    edge: &common::Edge,
    input: &ast::Input,
    statements: &[ast::Statement],
) {
    let number = retrieve_input(&program.declarations, input).unwrap();
    program.instructions.push(Instruction::On {
        input: number,
        edge: *edge,
    });
    for statement in statements.iter() {
        handle_statement(program, statement);
    }
    program.instructions.push(Instruction::Pop);
}

#[cfg(test)]
mod tests {
    use crate::shal::ast;
    use crate::shal::ast::{IODeclaration, IODeclarations};
    use crate::shal::bytecode;
    use crate::shal::bytecode::Instruction;
    use crate::shal::common::{Edge, IsWas, Value};
    use crate::shal::compiler::compile;
    use std::collections::HashMap;

    #[test]
    fn test_compile() {
        let ast_program = ast::Program {
            declarations: IODeclarations {
                inputs: HashMap::from([
                    (
                        "button_downstairs".try_into().unwrap(),
                        IODeclaration { pin: 0.try_into().unwrap(), name: None },
                    ),
                    (
                        "button_upstairs".try_into().unwrap(),
                        IODeclaration { pin: 1.try_into().unwrap(), name: None },
                    ),
                ]),
                outputs: HashMap::from([
                    (
                        "light_downstairs".try_into().unwrap(),
                        IODeclaration { pin: 0.try_into().unwrap(), name: None },
                    ),
                    (
                        "light_upstairs".try_into().unwrap(),
                        IODeclaration { pin: 1.try_into().unwrap(), name: None },
                    ),
                    (
                        "light_stairs".try_into().unwrap(),
                        IODeclaration { pin: 2.try_into().unwrap(), name: None },
                    ),
                ]),
            },
            statements: vec![
                ast::Statement::Event {
                    edge: Edge::Rising,
                    input: ast::Input::Entity("button_downstairs".try_into().unwrap()),
                    statements: vec![ast::Statement::Action(ast::Action::Toggle(
                        ast::Output::Entity("light_downstairs".try_into().unwrap()),
                    ))],
                },
                ast::Statement::Event {
                    edge: Edge::Rising,
                    input: ast::Input::Entity("button_upstairs".try_into().unwrap()),
                    statements: vec![ast::Statement::Action(ast::Action::Toggle(
                        ast::Output::Entity("light_upstairs".try_into().unwrap()),
                    ))],
                },
                ast::Statement::IfElse(
                    ast::Condition::Or(
                        Box::new(ast::Condition::Output(
                            ast::Output::Entity("light_downstairs".try_into().unwrap()),
                            IsWas::Is,
                            Value::High,
                        )),
                        Box::new(ast::Condition::Output(
                            ast::Output::Entity("light_upstairs".try_into().unwrap()),
                            IsWas::Is,
                            Value::High,
                        )),
                    ),
                    vec![ast::Statement::Action(ast::Action::Set(
                        ast::Output::Entity("light_stairs".try_into().unwrap()),
                        Value::High,
                    ))],
                    vec![ast::Statement::Action(ast::Action::Set(
                        ast::Output::Entity("light_stairs".try_into().unwrap()),
                        Value::Low,
                    ))],
                ),
            ],
        };
        let bytecode_program = compile(&ast_program);

        assert_eq!(
            &Ok(bytecode::Program {
                declarations: IODeclarations {
                    inputs: HashMap::from([
                        (
                            "button_downstairs".try_into().unwrap(),
                            IODeclaration { pin: 0.try_into().unwrap(), name: None }
                        ),
                        (
                            "button_upstairs".try_into().unwrap(),
                            IODeclaration { pin: 1.try_into().unwrap(), name: None }
                        ),
                    ]),
                    outputs: HashMap::from([
                        (
                            "light_downstairs".try_into().unwrap(),
                            IODeclaration { pin: 0.try_into().unwrap(), name: None }
                        ),
                        (
                            "light_upstairs".try_into().unwrap(),
                            IODeclaration { pin: 1.try_into().unwrap(), name: None }
                        ),
                        (
                            "light_stairs".try_into().unwrap(),
                            IODeclaration { pin: 2.try_into().unwrap(), name: None }
                        ),
                    ]),
                },
                instructions: vec![
                    Instruction::On {
                        input: 0.try_into().unwrap(),
                        edge: Edge::Rising,
                    },
                    Instruction::Toggle { output: 0.try_into().unwrap() },
                    Instruction::Pop,
                    Instruction::On {
                        input: 1.try_into().unwrap(),
                        edge: Edge::Rising,
                    },
                    Instruction::Toggle { output: 1.try_into().unwrap() },
                    Instruction::Pop,
                    Instruction::If {
                        number: 0.try_into().unwrap(),
                        value: Value::High,
                        is_was: IsWas::Is,
                        in_out: bytecode::InOut::Output,
                    },
                    Instruction::If {
                        number: 1.try_into().unwrap(),
                        value: Value::High,
                        is_was: IsWas::Is,
                        in_out: bytecode::InOut::Output,
                    },
                    Instruction::Or,
                    Instruction::Set {
                        output: 2.try_into().unwrap(),
                        value: Value::High,
                    },
                    Instruction::Not,
                    Instruction::Set {
                        output: 2.try_into().unwrap(),
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
