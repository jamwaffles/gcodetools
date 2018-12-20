use crate::token::{token, Token};
use crate::Span;
use nom::types::CompleteByteSlice;
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
        let raw = Span::new(CompleteByteSlice(b"G54\nG55"));

        assert_parse!(
            line,
            raw,
            Line {
                span: Span::new(CompleteByteSlice(b"")),
                tokens: vec![Token {
                    span: Span::new(CompleteByteSlice(b"")),
                    token: TokenType::GCode(GCode {
                        span: Span::new(CompleteByteSlice(b"")),
                        code: 54.0
                    })
                }]
            },
            // Remaining
            Span {
                offset: 4,
                line: 2,
                fragment: CompleteByteSlice(b"G55")
            }
        );
    }
}
