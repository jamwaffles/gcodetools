mod gcodes;
mod helpers;
mod mcodes;
mod othercodes;

use nom;
use nom::types::CompleteByteSlice;

use self::gcodes::*;
use self::helpers::*;
use self::mcodes::*;
use self::othercodes::*;

pub struct Tokenizer<'a> {
    program_string: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new_from_str(program_string: &'a str) -> Self {
        Tokenizer { program_string }
    }

    pub fn tokenize(&self) -> Result<(CompleteByteSlice, Program), nom::Err<CompleteByteSlice>> {
        program(CompleteByteSlice(self.program_string.as_bytes()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Units(Units),
    DistanceMode(DistanceMode),
    PathBlending(PathBlending),
    CutterCompensation(CutterCompensation),
    RapidMove,
    LinearMove,
    Coord(Vec9),
    ToolSelect(u32),
    ToolChange,
    PlaneSelect(Plane),
    SpindleRotation(SpindleRotation),
    SpindleSpeed(i32),
    FeedRate(f32),
    LineNumber(u32),
    Coolant(Coolant),
}

pub type Program = Vec<Token>;

named!(token<CompleteByteSlice, Token>,
    alt_complete!(
        gcode |
        mcode |
        othercode |
        coord |
        comment
    )
);

named!(tokens<CompleteByteSlice, Vec<Token>>, ws!(many0!(token)));

named!(program<CompleteByteSlice, Program>,
    alt!(
        ws!(delimited!(tag!("%"), tokens, tag!("%"))) |
        ws!(terminated!(tokens, alt!(tag!("M30") | tag!("M2"))))
    )
);

// Note: programs are either dlimited by % signs or stop at M2/M30. Anything after a trailing %/M2/
// M30 MUST be ignored

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

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
                Cbs(b"N50"),
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
                    Token::FeedRate(500.0f32),
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
                // Ignore anything after last %
                Cbs(b"G0 Z10\n"),
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
                ]
            ))
        );
    }
}
