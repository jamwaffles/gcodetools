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
        tokens: terminated!(
            sep!(space0, many0!(token)),
            alt!(line_ending | eof!())
        ) >>
        (Line { tokens, span })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{GCode, TokenType};

    #[test]
    fn parse_multiple_spaced_tokens() {
        let raw = span!(b"G54 G55  G56\tG57");

        assert_parse!(
            parser = line,
            input = raw,
            expected = Line {
                span: empty_span!(),
                tokens: vec![
                    Token {
                        span: empty_span!(),
                        token: TokenType::GCode(GCode {
                            span: empty_span!(),
                            code: 54.0
                        })
                    },
                    Token {
                        span: empty_span!(offset = 4),
                        token: TokenType::GCode(GCode {
                            span: empty_span!(offset = 4),
                            code: 55.0
                        })
                    },
                    Token {
                        span: empty_span!(offset = 9),
                        token: TokenType::GCode(GCode {
                            span: empty_span!(offset = 9),
                            code: 56.0
                        })
                    },
                    Token {
                        span: empty_span!(offset = 13),
                        token: TokenType::GCode(GCode {
                            span: empty_span!(offset = 13),
                            code: 57.0
                        })
                    }
                ]
            },
            remaining = empty_span!(offset = 16)
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
                    token: TokenType::GCode(GCode {
                        span: empty_span!(),
                        code: 54.0
                    })
                }]
            },
            remaining = span!(b"G55", offset = 4, line = 2)
        );
    }
}
