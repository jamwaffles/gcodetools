use crate::Span;
use nom::*;

named!(pub recognize_ngc_float<Span, Span>,
    recognize!(
        tuple!(
            opt!(alt!(char!('+') | char!('-'))),
            alt!(
                value!((), tuple!(digit, opt!(pair!(char!('.'), opt!(digit))))) |
                value!((), tuple!(char!('.'), digit))
            )
        )
    )
);

named!(pub ngc_float<Span, f32>,
    flat_map!(
        recognize_ngc_float,
        parse_to!(f32)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_float() {
        assert_parse!(
            parser = ngc_float;
            input = span!(b"5.0"),
                    span!(b"5."),
                    span!(b".5"),
                    span!(b"-32.0"),
                    span!(b"1000"),
                    span!(b"+64.2");
            expected =
                    5.0,
                    5.0,
                    0.5,
                    -32.0,
                    1000.0,
                    64.2
        );
    }

    #[test]
    #[should_panic]
    fn fail_float_exponent() {
        let (remaining, _) = ngc_float(span!(b"5.0e10")).unwrap();

        assert_eq!(remaining, empty_span!(offset = 0));
    }
}
