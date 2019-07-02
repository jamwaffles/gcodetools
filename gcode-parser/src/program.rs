use crate::line::{line, Line};
use crate::token::Token;
use expression::gcode::expression;
use nom::{
    character::streaming::{char, line_ending},
    combinator::opt,
    error::{context, convert_error, ParseError, VerboseError},
    multi::many0,
    sequence::tuple,
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
        // let input = Span::new(CompleteByteSlice(content.as_bytes()));

        // let (remaining, program) = program(input).map_err(|e| {
        //     let message = match e {
        //         Err::Error(Context::Code(remaining, _inner_e)) => format_parse_error!(
        //             remaining,
        //             inner_e,
        //             Span::new(CompleteByteSlice(content.as_bytes()))
        //         ),
        //         _ => format!("Parse execution failed: {:?}", e.into_error_kind()),
        //     };

        //     io::Error::new(io::ErrorKind::Other, message)
        // })?;

        // if remaining.input_len() > 0 {
        //     Err(io::Error::new(
        //         io::ErrorKind::Other,
        //         format_parse_error!(
        //             remaining,
        //             io::Error::new(
        //                 io::ErrorKind::Other,
        //                 "Could not parse complete program".to_string()
        //             ),
        //             input
        //         ),
        //     ))
        // } else {
        //     Ok(program)
        // }

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
                        _ => String::from("Failed to parse for unknown reason"),
                    },
                )
            })
            .and_then(|(remaining, result)| {
                if remaining.len() > 0 {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("{} remaining bytes to parse", remaining.len()),
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

// named!(pub program<Span, Program>,
//     do_parse!(
//         start: position!() >>
//         opt!(line_with!(char!('%'))) >>
//         lines: many0!(line) >>
//         opt!(line_with!(char!('%'))) >>
//         multispace0 >>
//         end: position!() >>
//         ({
//             Program {
//                 start,
//                 end,
//                 lines
//             }
//         })
//     )
// );

pub fn program<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Program, E> {
    // TODO: `line_with` function
    let (i, _) = opt(tuple((char('%'), line_ending)))(i)?;

    let (i, lines) = many0(line)(i)?;

    let (i, _) = opt(tuple((char('%'), opt(line_ending))))(i)?;

    Ok((i, Program { lines }))
}

#[cfg(test)]
mod tests {
    use super::{program, Line, Program};
    use crate::assert_parse;
    use crate::coord;
    use crate::token::{Coord, GCode, MCode, Token, TokenType};

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
                    }
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
                                token: TokenType::MCode(MCode::EndProgram)
                            }
                        ],
                        ..Line::default()
                    }
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
            input = "G0 X0 Y0 Z0\nG1 X1 Y1 Z1\nM30";
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
                    }
                ]
            };
        );
    }

    #[test]
    fn empty_lines() {
        assert_parse!(
            parser = program;
            input = "\n\n\nM2";
            expected = Program {
                lines: vec![
                    Line::default(),
                    Line {
                        tokens: vec![Token {
                            token: TokenType::MCode(MCode::EndProgram)
                        }],
                        ..Line::default()
                    },
                ]
            };
        );
    }

    #[test]
    fn blank_lines() {
        assert_parse!(
            parser = program;
            input = "G0\n\nG1\nM2";
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
