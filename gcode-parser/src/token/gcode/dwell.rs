use crate::{map_code, Span};
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
            float
        ),
        |time| Dwell { time }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dwell_decimal() {
        assert_parse!(
            parser = dwell,
            input = span!(b"G4 P0.01"),
            expected = Dwell { time: 0.01 },
            remaining = empty_span!(offset = 8)
        );
    }

    #[test]
    fn dwell_integer() {
        assert_parse!(
            parser = dwell,
            input = span!(b"G4 P3"),
            expected = Dwell { time: 3.0 },
            remaining = empty_span!(offset = 5)
        );
    }

    #[test]
    #[should_panic]
    fn dwell_p_value_required() {
        dwell(span!(b"G4")).unwrap();
    }
}
