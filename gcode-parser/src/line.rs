use crate::token::{block_delete, line_number, token, Token};
use nom::{
    character::complete::{line_ending, space0},
    combinator::{complete, map, opt},
    error::{context, ParseError},
    multi::many0,
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    // pub(crate) span: Span,
    pub(crate) tokens: Vec<Token>,
}

impl Line {
    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.tokens.iter()
    }
}

impl Default for Line {
    fn default() -> Self {
        Self { tokens: Vec::new() }
    }
}

pub fn line<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    let (i, (block_delete, line_number, line_tokens)) = context(
        "line",
        complete(delimited(
            space0,
            tuple((
                opt(terminated(block_delete, space0)),
                opt(terminated(line_number, space0)),
                many0(terminated(token, space0)),
            )),
            space0,
        )),
    )(i)?;

    let tokens: Vec<Token> = vec![block_delete, line_number]
        .into_iter()
        .filter_map(|t| t)
        .chain(line_tokens.into_iter())
        .collect();

    Ok((i, Line { tokens }))
}

/// A list of newline-separated token lists without trailing newline
pub fn lines<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Vec<Line>, E> {
    map(pair(lines_with_newline, line), |(mut lines, last)| {
        lines.push(last);

        lines
    })(i)
}

/// Like `line`, but requires a trailing newline
pub fn lines_with_newline<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<Line>, E> {
    many0(terminated(line, line_ending))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use crate::token::{
        CenterFormatArc, Comment, CutterCompensation, GCode, MCode, TokenType, WorkOffset,
        WorkOffsetValue,
    };

    #[test]
    fn block_delete() {
        assert_parse!(
            parser = line;
            input =
                "/G54\n",
                "/ G55\n"
            ;
            expected =
                Line {
                    tokens: vec![
                        Token {
                            token: TokenType::BlockDelete
                        },
                        Token {
                            token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                                offset: WorkOffsetValue::G54,
                            }))
                        }
                    ],
                    ..Line::default()
                },
                Line {
                    tokens: vec![
                        Token {
                            token: TokenType::BlockDelete
                        },
                        Token {
                            token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                                offset: WorkOffsetValue::G55,
                            }))
                        }
                    ],
                    ..Line::default()
                }
            ;
            remaining =
                "\n",
                "\n"
            ;
        );
    }

    #[test]
    fn parse_multiple_spaced_tokens() {
        assert_parse!(
            parser = line;
            input = "G54 G55  G56\tG57\n";
            expected = Line {
                tokens: vec![
                    Token {
                        token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                            offset: WorkOffsetValue::G54,
                        }))
                    },
                    Token {
                        token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                            offset: WorkOffsetValue::G55,
                        }))
                    },
                    Token {
                        token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                            offset: WorkOffsetValue::G56,
                        }))
                    },
                    Token {
                        token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                            offset: WorkOffsetValue::G57,
                        }))
                    }
                ],
                ..Line::default()
            };
            remaining = "\n"
        );
    }

    #[test]
    fn arc() {
        assert_parse!(
            parser = line;
            input = "G3 X-2.4438 Y-0.2048 I-0.0766 J0.2022\n";
            expected = Line {
                tokens: vec![
                    Token {
                        token: TokenType::GCode(GCode::CounterclockwiseArc)
                    },
                    Token {
                        token: TokenType::CenterFormatArc(CenterFormatArc {
                            x: Some((-2.4438f32).into()),
                            y: Some((-0.2048f32).into()),
                            i: Some((-0.0766f32).into()),
                            j: Some((0.2022f32).into()),
                            ..CenterFormatArc::default()
                        })
                    }
                ],
                ..Line::default()
            };
            remaining = "\n"
        );
    }

    #[test]
    fn end_program() {
        assert_parse!(
            parser = line;
            input = "M2\n";
            expected = Line {
                tokens: vec![
                    Token {
                        token: TokenType::MCode(MCode::EndProgram)
                    },
                ],
                ..Line::default()
            };
            remaining = "\n"
        );
    }

    // TODO: Delete. Line func doesn't consume ending anymore
    // #[test]
    // fn consume_line_and_ending() {
    //     assert_parse!(
    //         parser = line;
    //         input = "G54\nG55";
    //         expected = Line {
    //             tokens: vec![Token {
    //                 token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
    //                     offset: WorkOffsetValue::G54,
    //                 }))
    //             }],
    //             ..Line::default()
    //         };
    //         remaining = "G55"
    //     );
    // }

    #[test]
    fn ignore_surrounding_whitespace() {
        assert_parse!(
            parser = line;
            input = " G54 \nG55";
            expected = Line {
                tokens: vec![Token {
                    token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                        offset: WorkOffsetValue::G54,
                    }))
                }],
                ..Line::default()
            };
            remaining = "\nG55"
        );
    }

    #[test]
    fn empty_line() {
        assert_parse!(
            parser = line;
            input = "\n";
            expected = Line::default();
            remaining = "\n"
        );
    }

    #[test]
    fn empty_lines() {
        assert_parse!(
            parser = line;
            input = "\n\n";
            expected = Line::default();
            remaining = "\n\n"
        );
    }

    #[test]
    fn line_comment() {
        assert_parse!(
            parser = line;
            input = "; Line comment\nG55";
            expected = Line {
                tokens: vec![Token {
                    token: TokenType::Comment(Comment {
                        text: "Line comment".to_string()
                    })
                }],
                ..Line::default()
            };
            remaining = "\nG55"
        );
    }

    // TODO: Decide if this needs to be supported or not
    #[test]
    #[ignore]
    fn or_eof() {
        assert_parse!(
            parser = line;
            input = "G55";
            expected = Line {
                tokens: vec![Token {
                    token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                        offset: WorkOffsetValue::G55,
                    }))
                }],
                ..Line::default()
            };
        );
    }

    #[test]
    fn token_and_comment() {
        assert_parse!(
            parser = line;
            input = "G40 (disable tool radius compensation)\r\n";
            expected = Line {
                tokens: vec![Token {
                    token: TokenType::GCode(GCode::CutterCompensation(CutterCompensation::Off))
                }, Token {
                    token: TokenType::Comment(Comment {
                        text: "disable tool radius compensation".into()
                    })
                }],
                ..Line::default()
            };
            remaining = "\r\n"
        );
    }
}
