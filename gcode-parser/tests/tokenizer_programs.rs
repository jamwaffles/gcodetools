extern crate gcode_parser;
extern crate nom;

use gcode_parser::tokenizer::prelude::*;
use gcode_parser::tokenizer::*;
use nom::types::CompleteByteSlice as Cbs;

const EMPTY: Cbs = Cbs(b"");

#[test]
fn it_handles_program_endings() {
    let programs = vec![
        "%\nG0 Z0\n%\n",
        "G0 Z0\n%\n",
        "G0 Z0\nM2\n",
        "G0 Z0\nM30\n",
        "G0 Z0\nM30\n%\n",
        "G0 Z0\nM2\n%\n",
        "%\nG0 Z0\nM30\n%\n",
        "%\nG0 Z0\nM2\n%\n",
    ];

    for p in programs {
        assert!(program(Cbs(p.as_bytes())).is_ok());
    }
}

#[test]
fn it_parses_block_deletes() {
    let input = r#"G21
/G0 X0 Y1 Z10 F500
G0 X3 Y4"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![
                Token::Units(Units::Mm),
                Token::BlockDelete(vec![
                    Token::RapidMove,
                    Token::Coord(Vec9 {
                        x: Some(0.0),
                        y: Some(1.0),
                        z: Some(10.0),
                        ..Default::default()
                    }),
                    Token::FeedRate(500.0.into()),
                ]),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    x: Some(3.0),
                    y: Some(4.0),
                    ..Default::default()
                }),
            ]
        ))
    );
}

#[test]
fn it_parses_programs_with_line_numbers() {
    let input = r#"N10 G21
N20 G0 x0 y0 z0
N30 G1 Z10
N40 M30
N50"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![
                Token::LineNumber(10),
                Token::Units(Units::Mm),
                Token::LineNumber(20),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    x: Some(0.0),
                    y: Some(0.0),
                    z: Some(0.0),
                    ..Default::default()
                }),
                Token::LineNumber(30),
                Token::LinearMove,
                Token::Coord(Vec9 {
                    z: Some(10.0),
                    ..Default::default()
                }),
                Token::LineNumber(40),
                Token::EndProgram,
                Token::LineNumber(50),
            ]
        ))
    );
}

#[test]
fn it_parses_a_program() {
    let input = r#"G21
G0 f500 x0 y0 z0
G1 Z10
M2
"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![
                Token::Units(Units::Mm),
                Token::RapidMove,
                Token::FeedRate(500.0.into()),
                Token::Coord(Vec9 {
                    x: Some(0.0),
                    y: Some(0.0),
                    z: Some(0.0),
                    ..Default::default()
                }),
                Token::LinearMove,
                Token::Coord(Vec9 {
                    z: Some(10.0),
                    ..Default::default()
                }),
                Token::EndProgram,
            ]
        ))
    );
}

#[test]
fn it_parses_a_program_ending_with_m30() {
    let input = r#"G21
G0 x0 y0 z0
G1 Z10
M30
"#;

    assert_eq!(
        program(Cbs(input.as_bytes())),
        Ok((
            EMPTY,
            vec![
                Token::Units(Units::Mm),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    x: Some(0.0),
                    y: Some(0.0),
                    z: Some(0.0),
                    ..Default::default()
                }),
                Token::LinearMove,
                Token::Coord(Vec9 {
                    z: Some(10.0),
                    ..Default::default()
                }),
                Token::EndProgram,
            ]
        ))
    );
}

#[test]
fn it_parses_a_percent_delimited_program() {
    let percents = r#"%
G21
G0 x0 y0 z0
G1 Z10
%
G0 Z10
"#;

    let percents_program = program(Cbs(percents.as_bytes()));

    assert_eq!(
        percents_program,
        Ok((
            EMPTY,
            vec![
                Token::Units(Units::Mm),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    x: Some(0.0),
                    y: Some(0.0),
                    z: Some(0.0),
                    ..Default::default()
                }),
                Token::LinearMove,
                Token::Coord(Vec9 {
                    z: Some(10.0),
                    ..Default::default()
                }),
                Token::EndProgram,
                Token::RapidMove,
                Token::Coord(Vec9 {
                    z: Some(10.0),
                    ..Default::default()
                }),
            ]
        ))
    );
}

#[test]
fn it_parses_distinct_moves() {
    let moves = r#"G1 x10 y20
x20 y30
G0 y30 z10
y40 z20
M2"#;

    let moves_program = program(Cbs(moves.as_bytes()));

    assert_eq!(
        moves_program,
        Ok((
            EMPTY,
            vec![
                Token::LinearMove,
                Token::Coord(Vec9 {
                    x: Some(10.0),
                    y: Some(20.0),
                    ..Default::default()
                }),
                Token::Coord(Vec9 {
                    x: Some(20.0),
                    y: Some(30.0),
                    ..Default::default()
                }),
                Token::RapidMove,
                Token::Coord(Vec9 {
                    y: Some(30.0),
                    z: Some(10.0),
                    ..Default::default()
                }),
                Token::Coord(Vec9 {
                    y: Some(40.0),
                    z: Some(20.0),
                    ..Default::default()
                }),
                Token::EndProgram,
            ]
        ))
    );
}

#[test]
fn it_parses_arcs() {
    let arcs = r#"G2X5.0417Y1.9427I-0.3979J0.3028
M2"#;

    let arcs_program = program(Cbs(arcs.as_bytes()));

    assert_eq!(
        arcs_program,
        Ok((
            EMPTY,
            vec![
                Token::ClockwiseArc,
                Token::CenterFormatArc(CenterFormatArc {
                    x: Some(Value::Float(5.0417)),
                    y: Some(Value::Float(1.9427)),
                    i: Some(Value::Float(-0.3979)),
                    j: Some(Value::Float(0.3028)),
                    ..Default::default()
                }),
                Token::EndProgram,
            ]
        ))
    );
}
