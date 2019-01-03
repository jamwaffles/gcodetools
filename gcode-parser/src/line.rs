use crate::token::{token, Token};
use common::parsing::Span;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) tokens: Vec<Token<'a>>,
}

impl<'a> Line<'a> {
    pub fn iter(&'a self) -> impl Iterator<Item = &'a Token<'a>> {
        self.tokens.iter()
    }
}

named!(pub line<Span, Line>,
    sep!(
        space0,
        do_parse!(
            span: position!() >>
            tokens: many0!(token) >>
            alt!(line_ending | eof!()) >>
            (Line { tokens, span })
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
    fn parse_multiple_spaced_tokens() {
        assert_parse!(
            parser = line;
            input = span!(b"G54 G55  G56\tG57\n");
            expected = Line {
                span: empty_span!(),
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
                ]
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
                span: empty_span!(),
                tokens: vec![
                    Token {
                        span: empty_span!(),
                        token: TokenType::GCode(GCode::CounterclockwiseArc)
                    },
                    Token {
                        span: empty_span!(offset = 3),
                        token: TokenType::CenterFormatArc(CenterFormatArc {
                            x: Some(-2.4438),
                            y: Some(-0.2048),
                            i: Some(-0.0766),
                            j: Some(0.2022),
                            ..CenterFormatArc::default()
                        })
                    }
                ]
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
                span: empty_span!(),
                tokens: vec![Token {
                    span: empty_span!(),
                    token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                        offset: WorkOffsetValue::G54,
                    }))
                }]
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
                }]
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
                span: empty_span!(),
                tokens: vec![Token {
                    span: empty_span!(),
                    token: TokenType::Comment(Comment {
                        text: "Line comment".to_string()
                    })
                }]
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
                span: empty_span!(),
                tokens: vec![Token {
                    span: empty_span!(),
                    token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                        offset: WorkOffsetValue::G55,
                    }))
                }]
            };
        );
    }

    #[test]
    fn token_and_comment() {
        assert_parse!(
            parser = line;
            input = span!(b"G40 (disable tool radius compensation)\r\n");
            expected = Line {
                span: empty_span!(),
                tokens: vec![Token {
                    span: empty_span!(),
                    token: TokenType::GCode(GCode::CutterCompensation(CutterCompensation::Off))
                }, Token {
                    span: empty_span!(offset = 4),
                    token: TokenType::Comment(Comment {
                        text: "disable tool radius compensation".into()
                    })
                }]
            };
            remaining = empty_span!(offset = 40, line = 2)
        );
    }
}
