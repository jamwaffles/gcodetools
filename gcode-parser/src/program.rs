use crate::line::{line, Line};
use crate::token::Token;
use crate::Span;
use nom::types::CompleteByteSlice;
use nom::*;
use std::io;

#[derive(Debug, PartialEq)]
pub enum ProgramMarkerType {
    Percent,
    M2,
    M30,
}

/// A complete GCode program
///
/// This can either be a top level program, or a sub-program included by file
#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    lines: Vec<Line<'a>>,
    marker_type: ProgramMarkerType,
}

impl<'a> Program<'a> {
    /// Parse a GCode program from a given string
    pub fn from_str(content: &'a str) -> Result<Self, io::Error> {
        let (remaining, program) = program(Span::new(CompleteByteSlice(content.as_bytes())))
            .map_err(|e| {
                let message = match e {
                    Err::Error(Context::Code(remaining, e)) => format_parse_error!(
                        remaining,
                        e,
                        Span::new(CompleteByteSlice(content.as_bytes()))
                    ),
                    _ => format!("Parse execution failed: {:?}", e.into_error_kind()),
                };

                io::Error::new(io::ErrorKind::Other, message)
            })?;

        // TODO: Return a better error type
        if remaining.input_len() > 0 {
            let line = remaining.line;
            let column = remaining.get_column();

            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Could not parse complete program, failed at line {} col {} (byte {} of {})",
                    line,
                    column,
                    remaining.input_len(),
                    content.len()
                ),
            ))
        } else {
            Ok(program)
        }
    }

    /// Get a flat iterator over every token in this program
    pub fn iter_flat(&'a self) -> impl Iterator<Item = &'a Token<'a>> {
        self.lines.iter().flat_map(|line| line.iter())
    }
}

// TODO: Replace char!() macro with "line containing X" macro
named!(percent_delimited_program<Span, Program>,
    map!(
        ws!(
            delimited!(
                opt!(char!('%')),
                many0!(line),
                terminated!(char!('%'), eof!())
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
                terminated!(tag_no_case!("M2"), eof!())
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
                terminated!(tag_no_case!("M30"), eof!())
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
                                token: TokenType::GCode(GCode::Rapid)
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
                                token: TokenType::GCode(GCode::Feed)
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
    fn parse_percent_terminated_program() {
        assert_parse_ok!(
            parser = program,
            input = span!(b"G0 X0 Y0 Z0\nG1 X1 Y1 Z1\n%")
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
                                token: TokenType::GCode(GCode::Rapid)
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
                                token: TokenType::GCode(GCode::Feed)
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
                                token: TokenType::GCode(GCode::Rapid)
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
                                token: TokenType::GCode(GCode::Feed)
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
