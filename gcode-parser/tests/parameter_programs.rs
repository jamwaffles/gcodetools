extern crate gcode_parser;
extern crate nom;

use gcode_parser::tokenizer::prelude::*;
use gcode_parser::tokenizer::*;
use nom::types::CompleteByteSlice as Cbs;

const EMPTY: Cbs = Cbs(b"");

// #[test]
// TODO
fn it_parses_block_deletes() {
    let input = r#"#1234 = 1.0
    #<named_var> = 100
    #<_global_var> = 10.0

G0 X0 Y0
G0 Z[#1234]
G0 Z[#<named_var>]
G0 Z[#<_global_var>]
"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![
                Token::ParameterAssignment((Parameter::Numbered(1234), 1.0)),
                Token::ParameterAssignment((Parameter::Named("named_var".into()), 100.0)),
                Token::ParameterAssignment((Parameter::Global("global_var".into()), 10.0)),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    x: Some(0.0),
                    y: Some(0.0),
                    ..Default::default()
                }),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    z: Some(0.0),
                    ..Default::default()
                }),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    z: Some(0.0),
                    ..Default::default()
                }),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    z: Some(0.0),
                    ..Default::default()
                }),
            ]
        ))
    );
}
