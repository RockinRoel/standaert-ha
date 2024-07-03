use crate::shal::compiler::compile;
use crate::shal::parser::parse;

#[test]
fn test_full_program() {
    let ast_program = parse(include_str!("../../static/standaertha.shal")).unwrap();
    let bytecode_program = compile(&ast_program).unwrap();

    assert_eq!(Ok(175), bytecode_program.check_program_length(None));
    assert_eq!(Ok(2), bytecode_program.check_stack_depth(None));
}

#[test]
fn test_short_with_entities() {
    let ast_program = parse(include_str!("../../static/short.shal")).unwrap();
    let bytecode_program = compile(&ast_program).unwrap();

    assert_eq!(Ok(6), bytecode_program.check_program_length(None));
    assert_eq!(Ok(1), bytecode_program.check_stack_depth(None));
}

#[test]
fn test_one_to_one_program() {
    let ast_program = parse(include_str!("../../static/1to1.shal")).unwrap();
    let bytecode_program = compile(&ast_program).unwrap();

    assert_eq!(Ok(161), bytecode_program.check_program_length(None));
    assert_eq!(Ok(1), bytecode_program.check_stack_depth(None));
}
