use crate::line::line;
use crate::line::Line;
use nom::types::CompleteByteSlice;
use nom::*;

#[derive(Debug, PartialEq)]
pub enum ProgramMarkerType {
    Percent,
    M2,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    lines: Vec<Line>,
    marker_type: ProgramMarkerType,
}

// TODO: Replace char!() macro with "line containing X" macro
named!(percent_delimited_program<CompleteByteSlice, Program>,
    map!(
        ws!(
            delimited!(
                char!('%'),
                many0!(line),
                char!('%')
            )
        ),
        |lines| {
            Program {
                lines,
                marker_type: ProgramMarkerType::Percent
            }
        }
    )
);

// TODO: Replace alt!(tag!() | tag!()) with tag_no_case!()
named!(m2_terminated_program<CompleteByteSlice, Program>,
    map!(
        ws!(
            terminated!(
                many0!(line),
                alt!(tag!("M2") | tag!("m2"))
            )
        ),
        |lines| {
            Program {
                lines,
                marker_type: ProgramMarkerType::M2
            }
        }
    )
);

named!(pub program<CompleteByteSlice, Program>,
    alt_complete!(
        percent_delimited_program |
        m2_terminated_program
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Coord, GCode, Token};

    #[test]
    fn parse_percent_delimited_program() {
        let program_text = r#"%
G0 X0 Y0 Z0
G1 X1 Y1 Z1
%"#;

        assert_parse!(
            program,
            CompleteByteSlice(program_text.as_bytes()),
            Program {
                lines: vec![
                    Line {
                        tokens: vec![
                            Token::GCode(GCode { code: 0.0 }),
                            Token::Coord(coord!(0.0, 0.0, 0.0))
                        ]
                    },
                    Line {
                        tokens: vec![
                            Token::GCode(GCode { code: 1.0 }),
                            Token::Coord(coord!(1.0, 1.0, 1.0))
                        ]
                    }
                ],
                marker_type: ProgramMarkerType::Percent
            }
        );
    }

    #[test]
    fn parse_m2_end_program() {
        let program_text = r#"G0 X0 Y0 Z0
G1 X1 Y1 Z1
M2"#;

        assert_parse!(
            program,
            CompleteByteSlice(program_text.as_bytes()),
            Program {
                lines: vec![
                    Line {
                        tokens: vec![
                            Token::GCode(GCode { code: 0.0 }),
                            Token::Coord(coord!(0.0, 0.0, 0.0))
                        ]
                    },
                    Line {
                        tokens: vec![
                            Token::GCode(GCode { code: 1.0 }),
                            Token::Coord(coord!(1.0, 1.0, 1.0))
                        ]
                    }
                ],
                marker_type: ProgramMarkerType::M2
            }
        );
    }
}
