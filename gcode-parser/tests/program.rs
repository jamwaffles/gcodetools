use gcode_parser::Program;

#[test]
fn test_10_xyz_rapids() {
    let program = r#"G0 X0 Y0 Z0
G1 X1 Y1 Z1
M2"#;

    let _parsed = Program::from_str(program).unwrap();
}

#[test]
#[should_panic]
fn fail_if_content_after_program() {
    let program = r#"G0 X0 Y0 Z0
G1 X1 Y1 Z1
M2
I shouldn't be here"#;

    let _parsed = Program::from_str(program).unwrap();
}
