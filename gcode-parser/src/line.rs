use crate::token::{block_delete, line_number, token, Token};
use common::parsing::Span;
use nom::types::CompleteByteSlice;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq, Clone)]
pub struct Line<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) tokens: Vec<Token<'a>>,
}

impl<'a> Line<'a> {
    pub fn iter(&'a self) -> impl Iterator<Item = &'a Token<'a>> {
        self.tokens.iter()
    }
}

impl<'a> Default for Line<'a> {
    fn default() -> Self {
        Self {
            span: Span::new(CompleteByteSlice(b"")),
            tokens: Vec::new(),
        }
    }
}

named!(pub line<Span, Line>,
    sep!(
        space0,
        do_parse!(
            span: position!() >>
            block_delete: opt!(block_delete) >>
            line_number: opt!(line_number) >>
            line_tokens: many0!(token) >>
            alt!(line_ending | eof!()) >>
            ({
                let pre: Vec<Token> = vec![block_delete, line_number]
                    .into_iter()
                    .filter_map(|t| t)
                    .chain(line_tokens.into_iter())
                    .collect();

                Line { tokens: pre, span }
            })
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{
        CenterFormatArc, Comment, CutterCompensation, GCode, TokenType, WorkOffset, WorkOffsetValue,
    };
    use common::{assert_parse, empty_span, span};

    #[test]
    fn block_delete() {
        assert_parse!(
            parser = line;
            input =
                span!(b"/G54\n"),
                span!(b"/ G55\n")
            ;
            expected =
                Line {
                    tokens: vec![
                        Token {
                            span: empty_span!(),
                            token: TokenType::BlockDelete
                        },
                        Token {
                            span: empty_span!(offset = 1),
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
                            span: empty_span!(),
                            token: TokenType::BlockDelete
                        },
                        Token {
                            span: empty_span!(offset = 2),
                            token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                                offset: WorkOffsetValue::G55,
                            }))
                        }
                    ],
                    ..Line::default()
                }
            ;
            remaining =
                empty_span!(offset = 5, line = 2),
                empty_span!(offset = 6, line = 2)
            ;
        );
    }

    #[test]
    fn parse_multiple_spaced_tokens() {
        assert_parse!(
            parser = line;
            input = span!(b"G54 G55  G56\tG57\n");
            expected = Line {
                tokens: vec![
                    Token {
                        span: empty_span!(),
                        token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                            offset: WorkOffsetValue::G54,
                        }))
                    },
                    Token {
                        span: empty_span!(offset = 4),
                        token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                            offset: WorkOffsetValue::G55,
                        }))
                    },
                    Token {
                        span: empty_span!(offset = 9),
                        token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                            offset: WorkOffsetValue::G56,
                        }))
                    },
                    Token {
                        span: empty_span!(offset = 13),
                        token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                            offset: WorkOffsetValue::G57,
                        }))
                    }
                ],
                ..Line::default()
            };
            remaining = empty_span!(offset = 17, line = 2)
        );
    }

    #[test]
    fn arc() {
        assert_parse!(
            parser = line;
            input = span!(b"G3 X-2.4438 Y-0.2048 I-0.0766 J0.2022\n");
            expected = Line {
                tokens: vec![
                    Token {
                        span: empty_span!(),
                        token: TokenType::GCode(GCode::CounterclockwiseArc)
                    },
                    Token {
                        span: empty_span!(offset = 3),
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
            remaining = empty_span!(offset = 38, line = 2)
        );
    }

    #[test]
    fn consume_line_and_ending() {
        assert_parse!(
            parser = line;
            input = span!(b"G54\nG55");
            expected = Line {
                tokens: vec![Token {
                    span: empty_span!(),
                    token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                        offset: WorkOffsetValue::G54,
                    }))
                }],
                ..Line::default()
            };
            remaining = span!(b"G55", offset = 4, line = 2)
        );
    }

    #[test]
    fn ignore_surrounding_whitespace() {
        assert_parse!(
            parser = line;
            input = span!(b" G54 \nG55");
            expected = Line {
                span: empty_span!(offset = 1),
                tokens: vec![Token {
                    span: empty_span!(offset = 1),
                    token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                        offset: WorkOffsetValue::G54,
                    }))
                }],
                ..Line::default()
            };
            remaining = span!(b"G55", offset = 6, line = 2)
        );
    }

    #[test]
    fn line_comment() {
        assert_parse!(
            parser = line;
            input = span!(b"; Line comment\nG55");
            expected = Line {
                tokens: vec![Token {
                    span: empty_span!(),
                    token: TokenType::Comment(Comment {
                        text: "Line comment".to_string()
                    })
                }],
                ..Line::default()
            };
            remaining = span!(b"G55", offset = 15, line = 2)
        );
    }

    #[test]
    fn or_eof() {
        assert_parse!(
            parser = line;
            input = span!(b"G55");
            expected = Line {
                tokens: vec![Token {
                    span: empty_span!(),
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
            input = span!(b"G40 (disable tool radius compensation)\r\n");
            expected = Line {
                tokens: vec![Token {
                    span: empty_span!(),
                    token: TokenType::GCode(GCode::CutterCompensation(CutterCompensation::Off))
                }, Token {
                    span: empty_span!(offset = 4),
                    token: TokenType::Comment(Comment {
                        text: "disable tool radius compensation".into()
                    })
                }],
                ..Line::default()
            };
            remaining = empty_span!(offset = 40, line = 2)
        );
    }
}
