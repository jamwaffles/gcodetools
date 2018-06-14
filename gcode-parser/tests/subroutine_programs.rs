extern crate gcode_parser;
extern crate nom;

use gcode_parser::expression::*;
use gcode_parser::subroutine::*;
use gcode_parser::tokenizer::prelude::*;
use gcode_parser::tokenizer::*;
use nom::types::CompleteByteSlice as Cbs;

const EMPTY: Cbs = Cbs(b"");

#[test]
fn it_parses_programs_with_numbered_subroutines() {
    let input = r#"o100 sub
          G54 G0 X0 Y0 Z0
        o100 endsub

        o100 call"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![
                Token::SubroutineDefinition(Subroutine {
                    name: SubroutineName::Number(100),
                    tokens: vec![
                        Token::GCode(GCode::WorkOffset(WorkOffset::G54)),
                        Token::GCode(GCode::RapidMove),
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
                }),
                Token::SubroutineCall(SubroutineCall {
                    name: SubroutineName::Number(100),
                    args: None,
                }),
            ]
        ))
    );
}

#[test]
fn it_parses_subroutine_calls_with_args() {
    let input = r#"o100 sub
          G54 G0 X0 Y0 Z0
        o100 endsub

        o100 call [50] [#<foobar> + 2]"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![
                Token::SubroutineDefinition(Subroutine {
                    name: SubroutineName::Number(100),
                    tokens: vec![
                        Token::GCode(GCode::WorkOffset(WorkOffset::G54)),
                        Token::GCode(GCode::RapidMove),
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
                }),
                Token::SubroutineCall(SubroutineCall {
                    name: SubroutineName::Number(100),
                    args: Some(vec![
                        vec![ExpressionToken::Literal(50.0)],
                        vec![
                            ExpressionToken::Parameter(Parameter::Named("foobar".into())),
                            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                            ExpressionToken::Literal(2.0),
                        ],
                    ]),
                }),
            ]
        ))
    );
}

#[test]
fn it_parses_named_subroutines() {
    let input = r#"o<foo_bar> sub
          G54 G0 X0 Y0 Z0
        o<foo_bar> endsub"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![Token::SubroutineDefinition(Subroutine {
                name: SubroutineName::External("foo_bar".into()),
                tokens: vec![
                    Token::GCode(GCode::WorkOffset(WorkOffset::G54)),
                    Token::GCode(GCode::RapidMove),
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
