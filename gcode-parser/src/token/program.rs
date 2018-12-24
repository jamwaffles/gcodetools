use crate::line::{line, Line};
use crate::Span;
use nom::*;

#[derive(Debug, PartialEq)]
pub enum ProgramMarkerType {
    Percent,
    M2,
    M30,
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

named!(m2_terminated_program<Span, Program>,
    map!(
        ws!(
            terminated!(
                many0!(line),
                tag_no_case!("M2")
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

// TODO: Dedupe M2 and M30
named!(m30_terminated_program<Span, Program>,
    map!(
        ws!(
            terminated!(
                many0!(line),
                tag_no_case!("M30")
            )
        ),
        |lines| {
            Program {
                lines,
                marker_type: ProgramMarkerType::M30
            }
        }
    )
);

named!(pub program<Span, Program>,
    alt_complete!(
        percent_delimited_program |
        m2_terminated_program |
        m30_terminated_program
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
                                token: TokenType::Coord(coord!(
                                    empty_span!(offset = 5, line = 2),
                                    0.0,
                                    0.0,
                                    0.0
                                ))
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
                                token: TokenType::Coord(coord!(
                                    empty_span!(offset = 17, line = 3),
                                    1.0,
                                    1.0,
                                    1.0
                                ))
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
                                token: TokenType::Coord(coord!(
                                    empty_span!(offset = 3),
                                    0.0,
                                    0.0,
                                    0.0
                                ))
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
                                token: TokenType::Coord(coord!(
                                    empty_span!(offset = 15, line = 2),
                                    1.0,
                                    1.0,
                                    1.0
                                ))
                            }
                        ]
                    }
                ],
                marker_type: ProgramMarkerType::M2
            },
            remaining = empty_span!(offset = 26, line = 3)
        );
    }

    #[test]
    fn parse_m30_end_program() {
        let program_text = r#"G0 X0 Y0 Z0
G1 X1 Y1 Z1
M30"#;

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
                                token: TokenType::Coord(coord!(
                                    empty_span!(offset = 3),
                                    0.0,
                                    0.0,
                                    0.0
                                ))
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
                                token: TokenType::Coord(coord!(
                                    empty_span!(offset = 15, line = 2),
                                    1.0,
                                    1.0,
                                    1.0
                                ))
                            }
                        ]
                    }
                ],
                marker_type: ProgramMarkerType::M30
            },
            remaining = empty_span!(offset = 27, line = 3)
        );
    }
}
