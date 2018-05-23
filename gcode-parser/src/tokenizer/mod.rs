mod gcodes;
mod helpers;
mod mcodes;
mod othercodes;

use nom::types::CompleteByteSlice;

use self::gcodes::*;
use self::helpers::*;
use self::mcodes::*;
use self::othercodes::*;

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new_from_str() -> Self {
        Tokenizer {}
    }

    pub fn tokenize(&self) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Units(Units),
    DistanceMode(DistanceMode),
    PathBlending(PathBlending),
    CutterCompensation(CutterCompensation),
    RapidMove(Vec9),
    LinearMove(Vec9),
    ToolSelect(u32),
    ToolChange,
    PlaneSelect(Plane),
    SpindleRotation(SpindleRotation),
    SpindleSpeed(i32),
}

pub type Program = Vec<Token>;

named!(token<CompleteByteSlice, Token>,
    alt_complete!(
        comment |
        units |
        distance_mode |
        path_blending |
        cutter_compensation |
        rapid_move |
        linear_move |
        tool_number |
        tool_change |
        plane_select |
        spindle_rotation |
        spindle_speed
    )
);

named!(tokens<CompleteByteSlice, Vec<Token>>, many0!(token));

named!(program<CompleteByteSlice, Program>,
    alt_complete!(
        ws!(delimited!(tag!("%"), tokens, tag!("%"))) |
        ws!(terminated!(tokens, tag!("M2")))
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
    fn it_parses_a_program() {
        let input = r#"G21
G0 x0 y0 z0
G1 Z10
M2
"#;

        assert_eq!(
            program(Cbs(input.as_bytes())),
            Ok((
                EMPTY,
                vec![
                    Token::Units(Units::Mm),
                    Token::RapidMove(Vec9 {
                        x: Some(0.0),
                        y: Some(0.0),
                        z: Some(0.0),
                        ..Default::default()
                    }),
                    Token::LinearMove(Vec9 {
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
                    Token::RapidMove(Vec9 {
                        x: Some(0.0),
                        y: Some(0.0),
                        z: Some(0.0),
                        ..Default::default()
                    }),
                    Token::LinearMove(Vec9 {
                        z: Some(10.0),
                        ..Default::default()
                    }),
                ]
            ))
        );
    }
}
