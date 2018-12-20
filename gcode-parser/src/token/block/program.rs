use crate::line::line;
use crate::line::Line;
use crate::Span;
use nom::*;

#[derive(Debug, PartialEq)]
pub enum ProgramMarkerType {
    Percent,
    M2,
}

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    lines: Vec<Line<'a>>,
    marker_type: ProgramMarkerType,
}

// TODO: Replace char!() macro with "line containing X" macro
named!(percent_delimited_program<Span, Program>,
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
named!(m2_terminated_program<Span, Program>,
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

named!(pub program<Span, Program>,
    alt_complete!(
        percent_delimited_program |
        m2_terminated_program
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Coord, GCode, Token, TokenType};

    #[test]
    fn parse_percent_delimited_program() {
        let program_text = r#"%
G0 X0 Y0 Z0
G1 X1 Y1 Z1
%"#;

        assert_parse!(
            parser = program,
            input = span!(program_text.as_bytes()),
            expected = Program {
                lines: vec![
                    Line {
                        span: empty_span!(offset = 2, line = 2),
                        tokens: vec![
                            Token {
                                span: empty_span!(offset = 2, line = 2),
                                token: TokenType::GCode(GCode {
                                    span: empty_span!(offset = 2, line = 2),
                                    code: 0.0
                                })
                            },
                            Token {
                                span: empty_span!(offset = 5, line = 2),
                                token: TokenType::Coord(coord!(0.0, 0.0, 0.0))
                            }
                        ]
                    },
                    Line {
                        span: empty_span!(offset = 14, line = 3),
                        tokens: vec![
                            Token {
                                span: empty_span!(offset = 14, line = 3),
                                token: TokenType::GCode(GCode {
                                    span: empty_span!(offset = 14, line = 3),
                                    code: 1.0
                                })
                            },
                            Token {
                                span: empty_span!(offset = 17, line = 3),
                                token: TokenType::Coord(coord!(1.0, 1.0, 1.0))
                            }
                        ]
                    }
                ],
                marker_type: ProgramMarkerType::Percent
            },
            remaining = empty_span!(offset = 27, line = 4)
        );
    }

    #[test]
    fn parse_m2_end_program() {
        let program_text = r#"G0 X0 Y0 Z0
G1 X1 Y1 Z1
M2"#;

        assert_parse!(
            parser = program,
            input = span!(program_text.as_bytes()),
            expected = Program {
                lines: vec![
                    Line {
                        span: empty_span!(),
                        tokens: vec![
                            Token {
                                span: empty_span!(),
                                token: TokenType::GCode(GCode {
                                    span: empty_span!(),
                                    code: 0.0
                                })
                            },
                            Token {
                                span: empty_span!(offset = 3),
                                token: TokenType::Coord(coord!(0.0, 0.0, 0.0))
                            }
                        ]
                    },
                    Line {
                        span: empty_span!(offset = 12, line = 2),
                        tokens: vec![
                            Token {
                                span: empty_span!(offset = 12, line = 2),
                                token: TokenType::GCode(GCode {
                                    span: empty_span!(offset = 12, line = 2),
                                    code: 1.0
                                })
                            },
                            Token {
                                span: empty_span!(offset = 15, line = 2),
                                token: TokenType::Coord(coord!(1.0, 1.0, 1.0))
                            }
                        ]
                    }
                ],
                marker_type: ProgramMarkerType::M2
            },
            remaining = empty_span!(offset = 26, line = 3)
        );
    }
}
