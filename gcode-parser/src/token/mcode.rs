use crate::parsers::code_number;
use crate::Span;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq)]
pub struct MCode<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) code: f32,
}

named!(pub mcode<Span, MCode>,
    do_parse!(
        span: position!() >>
        code: preceded!(one_of!("Mm"), code_number) >>
        (MCode { span, code })
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
            expected = MCode {
                code: 99.0,
                span: empty_span!()
            },
            remaining = empty_span!(offset = 3)
        );
    }

    #[test]
    fn parse_single_decimal_mcode() {
        let raw = span!(b"M100.1");

        assert_parse!(
            parser = mcode,
            input = raw,
            expected = MCode {
                code: 100.1,
                span: empty_span!()
            },
            remaining = empty_span!(offset = 6)
        );
    }

}
