use crate::token::{token, Token};
use nom::types::CompleteByteSlice;
use nom::*;

#[derive(Debug, PartialEq)]
pub struct Line {
    pub(crate) tokens: Vec<Token>,
}

named!(pub line<CompleteByteSlice, Line>,
    map!(
        terminated!(
            sep!(space0, many0!(token)),
            line_ending
        ),
        |tokens| {
            Line { tokens }
        }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::GCode;

    #[test]
    fn consume_line_and_ending() {
        let raw = CompleteByteSlice(b"G54\nG55");

        assert_parse!(
            line,
            raw,
            Line {
                tokens: vec![Token::GCode(GCode { code: 54.0 })]
            },
            CompleteByteSlice(b"G55")
        );
    }
}
