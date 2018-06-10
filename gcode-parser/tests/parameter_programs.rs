extern crate gcode_parser;
extern crate nom;

use gcode_parser::tokenizer::prelude::*;
use gcode_parser::tokenizer::*;
use nom::types::CompleteByteSlice as Cbs;

const EMPTY: Cbs = Cbs(b"");

#[test]
fn it_parses_parameterised_programs() {
    let input = r#"#1234 = 1.0
    #<named_var> = 100
    #<_global_var> = 10.0

G0 X0 Y0
G0 Z#1234
G0 Z#<named_var>
G0 Z#<_global_var>
"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![
                Token::ParameterAssignment((Parameter::Numbered(1234), Value::Float(1.0))),
                Token::ParameterAssignment((
                    Parameter::Named("named_var".into()),
                    Value::Float(100.0),
                )),
                Token::ParameterAssignment((
                    Parameter::Global("global_var".into()),
                    Value::Float(10.0),
                )),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    x: Some(Value::Float(0.0)),
                    y: Some(Value::Float(0.0)),
                    ..Default::default()
                }),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    z: Some(Value::Parameter(Parameter::Numbered(1234))),
                    ..Default::default()
                }),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    z: Some(Value::Parameter(Parameter::Named("named_var".into()))),
                    ..Default::default()
                }),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    z: Some(Value::Parameter(Parameter::Global("global_var".into()))),
                    ..Default::default()
                }),
            ]
        ))
    );
}
