use crate::map_code;
use crate::parsers::ngc_float;
use common::parsing::Span;
use nom::*;

/// Dwell
#[derive(Debug, PartialEq, Clone)]
pub struct Dwell {
    /// The length of time in seconds to dwell for
    pub time: f32,
}

named!(pub dwell<Span, Dwell>,
    map_code!(
        "G4",
        preceded!(
            char_no_case!('P'),
            ngc_float
        ),
        |time| Dwell { time }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, assert_parse_ok, empty_span, span};

    #[test]
    fn dwell_decimal() {
        assert_parse!(
            parser = dwell;
            input = span!(b"G4 P0.01");
            expected = Dwell { time: 0.01 }
        );
    }

    #[test]
    fn leading_zero() {
        assert_parse!(
            parser = dwell;
            input = span!(b"G04P3");
            expected = Dwell { time: 3.0 }
        );
    }

    #[test]
    fn dwell_integer() {
        assert_parse!(
            parser = dwell;
            input = span!(b"G4 P3");
            expected = Dwell { time: 3.0 }
        );
    }

    #[test]
    #[should_panic]
    fn dwell_p_value_required() {
        dwell(span!(b"G4")).unwrap();
    }
}
