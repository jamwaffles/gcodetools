use crate::parsers::code_number;
use crate::Span;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq)]
pub enum MCode<'a> {
    Raw(RawMCode<'a>),
    SpindleForward(SpindleForward<'a>),
}

#[derive(Debug, PartialEq)]
pub struct RawMCode<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) code: f32,
}

#[derive(Debug, PartialEq)]
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

named!(pub spindle_forward<Span, SpindleForward>,
    positioned!(
        code!(
            "M3",
            flat_map!(
                preceded!(tag_no_case!("S"), digit),
                parse_to!(u32)
            )
        ),
        |(span, rpm)| {
            SpindleForward { span, rpm }
        }
    )
);

named!(pub mcode<Span, MCode>,
    alt_complete!(
        map!(spindle_forward, MCode::SpindleForward) |
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
        let raw = span!(b"M3 S1000");

        assert_parse!(
            parser = mcode,
            input = raw,
            expected = MCode::SpindleForward(SpindleForward {
                rpm: 1000u32,
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 8)
        );
    }
}
