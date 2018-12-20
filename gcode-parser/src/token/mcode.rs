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
        let raw = span!(b"M99");

        assert_parse!(
            parser = mcode,
            input = raw,
            expected = MCode { code: 99.0 },
            remaining = empty_span!(offset = 3)
        );
    }

    #[test]
    fn parse_single_decimal_mcode() {
        let raw = span!(b"M100.1");

        assert_parse!(
            parser = mcode,
            input = raw,
            expected = MCode { code: 100.1 },
            remaining = empty_span!(offset = 6)
        );
    }

}
