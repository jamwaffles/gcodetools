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

#[test]
fn comments() {
    let program = r#"( test parens comment )
(test parens comment)
; line comment
;line comment
G1 X1 Y1 Z1
M2"#;

    let _parsed = Program::from_str(program).unwrap();
}

#[test]
fn test_counterclockwise_arc() {
    let program = r#"G3 X-2.4438 Y-0.2048 I-0.0766 J0.2022
M2"#;

    let _parsed = Program::from_str(program);

    println!("{:#?}", _parsed);
}
