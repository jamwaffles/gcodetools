use crate::token::{token, Token};
use crate::Span;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) tokens: Vec<Token<'a>>,
}

named!(pub line<Span, Line>,
    do_parse!(
        span: position!() >>
        tokens: terminated!(
            sep!(space0, many0!(token)),
            line_ending
        ) >>
        (Line { tokens, span })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{GCode, TokenType};

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
