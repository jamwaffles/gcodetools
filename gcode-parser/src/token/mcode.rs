use crate::parsers::code_number;
use crate::Span;
use nom::types::CompleteByteSlice;
use nom::*;

#[derive(Debug, PartialEq)]
pub struct MCode {
    code: f32,
}

named!(pub mcode<Span, MCode>,
    map!(
       preceded!(one_of!("Mm"), code_number),
       |code| MCode { code }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_integer_mcode() {
        let raw = Span::new(CompleteByteSlice(b"M99"));

        assert_parse!(
            mcode,
            raw,
            MCode { code: 99.0 },
            // Remaining
            Span {
                offset: 3,
                line: 1,
                fragment: CompleteByteSlice(b"")
            }
        );
    }

    #[test]
    fn parse_single_decimal_mcode() {
        let raw = Span::new(CompleteByteSlice(b"M100.1"));

        assert_parse!(
            mcode,
            raw,
            MCode { code: 100.1 },
            // Remaining
            Span {
                offset: 6,
                line: 1,
                fragment: CompleteByteSlice(b"")
            }
        );
    }

}
