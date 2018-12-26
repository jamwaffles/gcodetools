use crate::token::{token, Token};
use crate::Span;
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
    do_parse!(
        span: position!() >>
        tokens: sep!(space0, many0!(token)) >>
        line_ending >>
        (Line { tokens, span })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Comment, GCode, TokenType, WorkOffset, WorkOffsetValue};

    #[test]
    fn parse_multiple_spaced_tokens() {
        let raw = span!(b"G54 G55  G56\tG57\n");

        assert_parse!(
            parser = line,
            input = raw,
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
            },
            remaining = empty_span!(offset = 17, line = 2)
        );
    }

    #[test]
    fn consume_line_and_ending() {
        let raw = span!(b"G54\nG55");

        assert_parse!(
            parser = line,
            input = raw,
            expected = Line {
                span: empty_span!(),
                tokens: vec![Token {
                    span: empty_span!(),
                    token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                        offset: WorkOffsetValue::G54,
                    }))
                }]
            },
            remaining = span!(b"G55", offset = 4, line = 2)
        );
    }

    #[test]
    fn line_comment() {
        assert_parse!(
            parser = line,
            input = span!(b"; Line comment\nG55"),
            expected = Line {
                span: empty_span!(),
                tokens: vec![Token {
                    span: empty_span!(),
                    token: TokenType::Comment(Comment {
                        text: "Line comment".to_string()
                    })
                }]
            },
            remaining = span!(b"G55", offset = 15, line = 2)
        );
    }
}
