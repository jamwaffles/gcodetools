use crate::line::{lines, Line};
use crate::token::Token;
use nom::{
    error::{context, convert_error, ParseError, VerboseError},
    IResult,
};
use std::io;

/// A complete GCode program
///
/// This can either be a top level program, or a sub-program included by file
#[derive(Debug, PartialEq)]
pub struct Program {
    // start: Span<'a>,
    // end: Span<'a>,
    lines: Vec<Line>,
}

impl Program {
    // TODO: Return a custom parse error type
    /// Parse a GCode program from a given string
    pub fn from_str(content: &str) -> Result<Self, io::Error> {
        // TODO: Format error helper function to move into common crate
        program::<VerboseError<&str>>(content)
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    match e {
                        nom::Err::Error(e) | nom::Err::Failure(e) => {
                            let e = convert_error(content, e);
                            println!("{}", e);
                            e
                        }
                        e => format!("Failed to parse: {:?}", e),
                    },
                )
            })
            .and_then(|(remaining, result)| {
                if remaining.len() > 0 {
                    let total_lines = content.lines().count();
                    let remaining_lines = remaining.lines().count();

                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "Failed at line {} ({} remaining bytes to parse)",
                            total_lines - remaining_lines + 1,
                            remaining.len()
                        ),
                    ))
                } else {
                    Ok(result)
                }
            })
    }

    /// Get a flat iterator over every token in this program
    pub fn iter_flat(&self) -> impl Iterator<Item = &Token> {
        self.lines.iter().flat_map(|line| line.iter())
    }
}

pub fn program<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Program, E> {
    let (i, lines) = context("program", lines)(i)?;

    Ok((i, Program { lines }))
}

#[cfg(test)]
mod tests {
    use super::{program, Line, Program};
    use crate::assert_parse;
    use crate::coord;
    use crate::token::{Coord, CutterCompensation, GCode, MCode, Token, TokenType};

    #[test]
    fn parse_percent_delimited_program() {
        assert_parse!(
            parser = program;
            input = "%\nG0 X0 Y0 Z0\nG1 X1 Y1 Z1\n%";
            expected = Program {
                lines: vec![
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::ProgramDelimiter
                            },
                        ],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::GCode(GCode::Rapid)
                            },
                            Token {
                                token: TokenType::Coord(coord!(0.0, 0.0, 0.0))
                            }
                        ],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::GCode(GCode::Feed)
                            },
                            Token {
                                token: TokenType::Coord(coord!(1.0, 1.0, 1.0))
                            }
                        ],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::ProgramDelimiter
                            },
                        ],
                        ..Line::default()
                    },
                ]
            };
        );
    }

    #[test]
    fn parse_percent_terminated_program() {
        assert_parse!(
            parser = program;
            input = "G0 X0 Y0 Z0\nG1 X1 Y1 Z1\n%";
            expected = Program {
                lines: vec![
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::GCode(GCode::Rapid)
                            },
                            Token {
                                token: TokenType::Coord(coord!(0.0, 0.0, 0.0))
                            }
                        ],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::GCode(GCode::Feed)
                            },
                            Token {
                                token: TokenType::Coord(coord!(1.0, 1.0, 1.0))
                            }
                        ],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::ProgramDelimiter
                            },
                        ],
                        ..Line::default()
                    },
                ]
            }
        );
    }

    #[test]
    fn parse_m2_end_program() {
        assert_parse!(
            parser = program;
            input = "G0 X0 Y0 Z0\nG1 X1 Y1 Z1\nM2";
            expected = Program {
                lines: vec![
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::GCode(GCode::Rapid)
                            },
                            Token {
                                token: TokenType::Coord(coord!(0.0, 0.0, 0.0))
                            }
                        ],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::GCode(GCode::Feed)
                            },
                            Token {
                                token: TokenType::Coord(coord!(1.0, 1.0, 1.0))
                            }
                        ],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::MCode(MCode::EndProgram)
                            }
                        ],
                        ..Line::default()
                    }
                ]
            };
        );
    }

    #[test]
    fn parse_m30_end_program() {
        assert_parse!(
            parser = program;
            input = "G0 X0 Y0 Z0\nG1 X1 Y1 Z1\nM30\n";
            expected = Program {
                lines: vec![
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::GCode(GCode::Rapid)
                            },
                            Token {
                                token: TokenType::Coord(coord!(0.0, 0.0, 0.0))
                            }
                        ],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::GCode(GCode::Feed)
                            },
                            Token {
                                token: TokenType::Coord(coord!(1.0, 1.0, 1.0))
                            }
                        ],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![
                            Token {
                                token: TokenType::MCode(MCode::EndProgramSwapPallets)
                            }
                        ],
                        ..Line::default()
                    },
                    Line::default()
                ]
            };
        );
    }

    #[test]
    fn empty_lines() {
        assert_parse!(
            parser = program;
            input = "\n\n\nM2\n";
            expected = Program {
                lines: vec![
                    Line::default(),
                    Line::default(),
                    Line::default(),
                    Line {
                        tokens: vec![Token {
                            token: TokenType::MCode(MCode::EndProgram)
                        }],
                        ..Line::default()
                    },
                    Line::default()
                ]
            };
        );
    }

    #[test]
    fn blank_lines() {
        assert_parse!(
            parser = program;
            input = "G0\nG1\n\nG41\nM2";
            expected = Program {
                lines: vec![
                    Line {
                        tokens: vec![Token {
                            token: TokenType::GCode(GCode::Rapid)
                        }],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![Token {
                            token: TokenType::GCode(GCode::Feed)
                        }],
                        ..Line::default()
                    },
                    Line::default(),
                    Line {
                        tokens: vec![Token {
                            token: TokenType::GCode(GCode::CutterCompensation(CutterCompensation::Left(None)))
                        }],
                        ..Line::default()
                    },
                    Line {
                        tokens: vec![Token {
                            token: TokenType::MCode(MCode::EndProgram)
                        }],
                        ..Line::default()
                    }
                ]
            };
        );
    }
}
