extern crate gcode_parser;
extern crate nom;

use gcode_parser::subroutine::*;
use gcode_parser::tokenizer::prelude::*;
use gcode_parser::tokenizer::*;
use nom::types::CompleteByteSlice as Cbs;

const EMPTY: Cbs = Cbs(b"");

#[test]
fn it_parses_programs_with_numbered_subroutines() {
    let input = r#"o100 sub
          G54 G0 X0 Y0 Z0
        o100 endsub"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![Token::SubroutineDefinition(Subroutine {
                name: SubroutineName::Number(100),
                tokens: vec![
                    Token::WorkOffset(WorkOffset::G54),
                    Token::RapidMove,
                    Token::Coord(Vec9 {
                        x: Some(Value::Float(0.0)),
                        y: Some(Value::Float(0.0)),
                        z: Some(Value::Float(0.0)),
                        a: None,
                        b: None,
                        c: None,
                        u: None,
                        v: None,
                        w: None,
                    }),
                ],
            })]
        ))
    );
}
