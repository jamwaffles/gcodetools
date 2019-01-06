//! Parse polar coordinates

use common::parsing::Span;
use expression::{parser::ngc_float_value, Value};
use nom::*;

/// A polar coordinate
#[derive(Debug, PartialEq, Clone)]
pub struct PolarCoord {
    /// Distance from origin
    pub distance: Value,
    /// Angle, starting at 0 on positive X axis. Positive direction is counterclockwise
    pub angle: Value,
}

named_attr!(#[doc = "Parse a polar coordinate"],
    pub polar<Span, PolarCoord>,
    map!(
        sep!(
            space0,
            permutation!(
                sep!(space0, preceded!(char_no_case!('@'), ngc_float_value)),
                sep!(space0, preceded!(char_no_case!('^'), ngc_float_value))
            )
        ),
        |(distance, angle)| {
            PolarCoord { distance, angle }
        }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};

    #[test]
    fn parse_polar() {
        assert_parse!(
            parser = polar;
            input = span!(b"@.5 ^90");
            expected = PolarCoord {
                distance: 0.5.into(),
                angle: 90.0.into()
            }
        );
    }
}
