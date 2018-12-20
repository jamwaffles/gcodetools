use crate::parsers::code_number;
use crate::Span;
use nom::types::CompleteByteSlice;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq)]
pub struct GCode<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) code: f32,
}

named!(pub gcode<Span, GCode>,
    do_parse!(
        span: position!() >>
        code: preceded!(one_of!("Gg"), code_number) >>
        (GCode { span, code })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom_locate::LocatedSpan;

    #[test]
    fn parse_single_integer_gcode() {
        let raw = Span::new(CompleteByteSlice(b"G54"));

        assert_parse!(
            gcode,
            raw,
            GCode {
                span: Span::new(CompleteByteSlice(b"")),
                code: 54.0
            },
            // Remaining
            LocatedSpan {
                offset: 3,
                line: 1,
                fragment: CompleteByteSlice(b"")
            }
        );
    }

    #[test]
    fn parse_single_decimal_gcode() {
        let raw = Span::new(CompleteByteSlice(b"G59.1"));

        assert_parse!(
            gcode,
            raw,
            GCode {
                span: Span::new(CompleteByteSlice(b"")),
                code: 59.1
            },
            // Remaining
            LocatedSpan {
                offset: 5,
                line: 1,
                fragment: CompleteByteSlice(b"")
            }
        );
    }
}
