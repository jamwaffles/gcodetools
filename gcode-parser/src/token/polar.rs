//! Parse polar coordinates

use crate::value::{preceded_value, Value};
use nom::{
    branch::permutation,
    bytes::streaming::tag_no_case,
    combinator::{map, opt},
    error::{context, ParseError},
    IResult,
};

/// A polar coordinate
#[derive(Debug, PartialEq, Clone)]
pub struct PolarCoord {
    /// Distance from origin (`@`)
    pub distance: Option<Value>,
    /// Angle, starting at 0 on positive X axis. Positive direction is counterclockwise (`^`)
    pub angle: Option<Value>,
}

// named_attr!(#[doc = "Parse a polar coordinate"],
//     pub polar<Span, PolarCoord>,
//     map_opt!(
//         sep!(
//             space0,
//             permutation!(
//                 sep!(space0, preceded!(char_no_case!('@'), ngc_float_value))?,
//                 sep!(space0, preceded!(char_no_case!('^'), ngc_float_value))?
//             )
//         ),
//         |(distance, angle): (Option<Value>, Option<Value>)| {
//             if distance.is_none() && angle.is_none() {
//                 None
//             } else {
//                 Some(PolarCoord { distance, angle })
//             }
//         }
//     )
// );

pub fn polar<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PolarCoord, E> {
    context(
        "polar coordinate",
        map(
            permutation((
                opt(preceded_value(tag_no_case("@"))),
                opt(preceded_value(tag_no_case("^"))),
            )),
            |(distance, angle)| PolarCoord { distance, angle },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;

    #[test]
    fn parse_polar() {
        assert_parse!(
            parser = polar;
            input = "@.5 ^90";
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
                "@.5",
                "^90"
            ;
            expected =
                PolarCoord { distance: Some(0.5.into()), angle: None },
                PolarCoord { distance: None, angle: Some(90.0.into()) }
            ;
        );
    }
}
