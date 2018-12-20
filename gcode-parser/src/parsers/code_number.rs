// TODO: Is there a better place to keep `CodeNumber`? In this file perhaps?
// use crate::token::CodeNumber;
use crate::Span;
use nom::types::CompleteByteSlice;
use nom::*;

pub type CodeNumber = f32;

named!(pub code_number<Span, CodeNumber>,
    flat_map!(
        recognize!(
            terminated!(
                digit1,
                opt!(terminated!(char!('.'), digit1))
            )
        ),
        parse_to!(f32)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_decimal() {
        assert_parse!(
            code_number,
            Span::new(CompleteByteSlice(b"59.1")),
            59.1f32,
            // Remaining
            Span {
                offset: 4,
                line: 1,
                fragment: CompleteByteSlice(b"")
            }
        );
    }

    #[test]
    fn parse_int() {
        assert_parse!(
            code_number,
            Span::new(CompleteByteSlice(b"54")),
            54.0f32,
            // Remaining
            Span {
                offset: 2,
                line: 1,
                fragment: CompleteByteSlice(b"")
            }
        );
    }
}
