use gcode_parser::{token::Token, Program};

#[test]
fn flat_iterator() {
    let program = r#"G0 X0 Y0 Z0
G1 X1 Y1 Z1
M2
"#;

    let parsed = Program::from_str(program).unwrap();

    let iter_result: Vec<&Token> = parsed.iter_flat().collect();

    assert_eq!(iter_result.len(), 5);
}
