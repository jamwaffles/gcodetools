use crate::parsers::code_number;
use crate::Span;
use nom::*;
use nom_locate::position;

/// A G-code
#[derive(Debug, PartialEq, Clone)]
pub enum GCode<'a> {
    /// A raw G-code with no arguments
    Raw(RawGCode<'a>),
}

/// A raw G-code
#[derive(Debug, PartialEq, Clone)]
pub struct RawGCode<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) code: f32,
}

named!(pub raw_gcode<Span, RawGCode>,
    // TODO: Benchmark do_parse! vs tuple!
    positioned!(
        preceded!(one_of!("Gg"), code_number),
        |(span, code)| RawGCode { span, code }
    )
);

named!(pub gcode<Span, GCode>,
    alt_complete!(
        map!(raw_gcode, GCode::Raw)
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
            expected = GCode::Raw(RawGCode {
                span: empty_span!(),
                code: 54.0
            }),
            remaining = empty_span!(offset = 3)
        );
    }

    #[test]
    fn parse_single_decimal_gcode() {
        let raw = span!(b"G59.1");

        assert_parse!(
            parser = gcode,
            input = raw,
            expected = GCode::Raw(RawGCode {
                span: empty_span!(),
                code: 59.1
            }),
            remaining = empty_span!(offset = 5)
        );
    }
}
