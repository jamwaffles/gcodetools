use crate::parsers::code_number;
use nom::types::CompleteByteSlice;
use nom::*;

#[derive(Debug, PartialEq)]
pub struct MCode {
    code: f32,
}

named!(pub mcode<CompleteByteSlice, MCode>,
    map!(
       preceded!(one_of!("Mm"), code_number),
       |code| MCode { code }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_integer_gcode() {
        let raw = CompleteByteSlice(b"M99");

        assert_parse!(mcode, raw, MCode { code: 99.0 });
    }

    #[test]
    fn parse_single_decimal_gcode() {
        let raw = CompleteByteSlice(b"M100.1");

        assert_parse!(mcode, raw, MCode { code: 100.1 });
    }

}
