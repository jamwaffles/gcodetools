use crate::parsers::code_number;
use crate::Span;
use nom::*;
use nom_locate::position;

/// An M-code
#[derive(Debug, PartialEq, Clone)]
pub enum MCode<'a> {
    /// A raw M-code that has no other arguments like M8 or M2
    Raw(RawMCode<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct RawMCode<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) code: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpindleForward<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) rpm: u32,
}

named!(pub raw_mcode<Span, RawMCode>,
    do_parse!(
        span: position!() >>
        code:  preceded!(tag_no_case!("M"), code_number) >>
        (RawMCode { span, code })
    )
);

named!(pub mcode<Span, MCode>,
    alt_complete!(
        map!(raw_mcode, MCode::Raw)
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
            expected = MCode::Raw(RawMCode {
                code: 99.0,
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 3)
        );
    }

    #[test]
    fn parse_single_decimal_mcode() {
        let raw = span!(b"M100.1");

        assert_parse!(
            parser = mcode,
            input = raw,
            expected = MCode::Raw(RawMCode {
                code: 100.1,
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 6)
        );
    }

    #[test]
    fn parse_spindle_forward() {
        let raw = span!(b"M3");

        assert_parse!(
            parser = mcode,
            input = raw,
            expected = MCode::Raw(RawMCode {
                code: 3.0,
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 2)
        );
    }
}
