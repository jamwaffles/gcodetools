use crate::parsers::code_number;
use nom::types::CompleteByteSlice;
use nom::*;

#[derive(Debug, PartialEq)]
pub struct GCode {
    pub(crate) code: f32,
}

named!(pub gcode<CompleteByteSlice, GCode>,
    map!(
       preceded!(one_of!("Gg"), code_number),
       |code| GCode { code }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_integer_gcode() {
        let raw = CompleteByteSlice(b"G54");

        assert_parse!(gcode, raw, GCode { code: 54.0 });
    }

    #[test]
    fn parse_single_decimal_gcode() {
        let raw = CompleteByteSlice(b"G59.1");

        assert_parse!(gcode, raw, GCode { code: 59.1 });
    }

}
