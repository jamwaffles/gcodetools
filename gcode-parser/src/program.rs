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
    None,
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
                    Err::Error(Context::Code(remaining, _e)) => format_parse_error!(
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

named!(pub program<Span, Program>,
    do_parse!(
        opt!(line_with!(char!('%'))) >>
        lines: many0!(line) >>
        marker_type: alt_complete!(
            map!(line_with!(char!('%')), |_| ProgramMarkerType::Percent) |
            map!(line_with!(tag_no_case!("M30")), |_| ProgramMarkerType::M30) |
            map!(line_with!(tag_no_case!("M2")), |_| ProgramMarkerType::M2) |
            map!(eof!(), |_| ProgramMarkerType::None)
        ) >>
        (Program { lines, marker_type })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Coord, GCode, Token, TokenType};

    #[test]
    fn parse_percent_delimited_program() {
        assert_parse!(
            parser = program;
            input = span!(b"%\nG0 X0 Y0 Z0\nG1 X1 Y1 Z1\n%");
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
                                token: TokenType::Coord(coord!(0.0, 0.0, 0.0))
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
                                token: TokenType::Coord(coord!(1.0, 1.0, 1.0))
                            }
                        ]
                    }
                ],
                marker_type: ProgramMarkerType::Percent
            };
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
        assert_parse!(
            parser = program;
            input = span!(b"G0 X0 Y0 Z0\nG1 X1 Y1 Z1\nM2");
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
                                token: TokenType::Coord(coord!(0.0, 0.0, 0.0))
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
                                token: TokenType::Coord(coord!(1.0, 1.0, 1.0))
                            }
                        ]
                    }
                ],
                marker_type: ProgramMarkerType::M2
            };
            remaining = empty_span!(offset = 26, line = 3)
        );
    }

    #[test]
    fn parse_m30_end_program() {
        assert_parse!(
            parser = program;
            input = span!(b"G0 X0 Y0 Z0\nG1 X1 Y1 Z1\nM30");
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
                                token: TokenType::Coord(coord!(0.0, 0.0, 0.0))
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
                                token: TokenType::Coord(coord!(1.0, 1.0, 1.0))
                            }
                        ]
                    }
                ],
                marker_type: ProgramMarkerType::M30
            };
            remaining = empty_span!(offset = 27, line = 3)
        );
    }
}
