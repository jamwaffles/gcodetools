//! Parse polar coordinates

use common::parsing::Span;
use expression::{parser::ngc_float_value, Value};
use nom::*;

/// A polar coordinate
#[derive(Debug, PartialEq, Clone)]
pub struct PolarCoord {
    /// Distance from origin
    pub distance: Option<Value>,
    /// Angle, starting at 0 on positive X axis. Positive direction is counterclockwise
    pub angle: Option<Value>,
}

named_attr!(#[doc = "Parse a polar coordinate"],
    pub polar<Span, PolarCoord>,
    map_opt!(
        sep!(
            space0,
            permutation!(
                sep!(space0, preceded!(char_no_case!('@'), ngc_float_value))?,
                sep!(space0, preceded!(char_no_case!('^'), ngc_float_value))?
            )
        ),
        |(distance, angle): (Option<Value>, Option<Value>)| {
            if distance.is_none() && angle.is_none() {
                None
            } else {
                Some(PolarCoord { distance, angle })
            }
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
                distance: Some(0.5.into()),
                angle: Some(90.0.into())
            }
        );
    }

    #[test]
    fn parse_polar_optional() {
        assert_parse!(
            parser = polar;
            input =
                span!(b"@.5"),
                span!(b"^90")
            ;
            expected =
                PolarCoord { distance: Some(0.5.into()), angle: None },
                PolarCoord { distance: None, angle: Some(90.0.into()) }
            ;
        );
    }
}
