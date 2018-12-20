use crate::parsers::code_number;
use crate::Span;
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

    #[test]
    fn parse_single_integer_gcode() {
        let raw = span!(b"G54");

        assert_parse!(
            parser = gcode,
            input = raw,
            expected = GCode {
                span: empty_span!(),
                code: 54.0
            },
            remaining = empty_span!(offset = 3)
        );
    }

    #[test]
    fn parse_single_decimal_gcode() {
        let raw = span!(b"G59.1");

        assert_parse!(
            parser = gcode,
            input = raw,
            expected = GCode {
                span: empty_span!(),
                code: 59.1
            },
            remaining = empty_span!(offset = 5)
        );
    }
}
