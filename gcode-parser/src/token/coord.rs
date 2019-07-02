//! Parse coordinates into a vector

use crate::value::{preceded_value, Value};

use nom::{
    branch::permutation,
    bytes::streaming::tag_no_case,
    combinator::{map_opt, opt},
    error::{context, ParseError},
    IResult,
};

/// A 9 dimensional `XYZABCUVW` coordinate
///
/// Any or all of the components of this coordinate can be `None`, however the parser requires at
/// least _one_ populated field to parse successfully.
#[derive(Debug, PartialEq, Clone)]
pub struct Coord {
    /// The optional X component of the coord
    pub x: Option<Value>,
    /// The optional Y component of the coord
    pub y: Option<Value>,
    /// The optional Z component of the coord
    pub z: Option<Value>,
    /// The optional A component of the coord
    pub a: Option<Value>,
    /// The optional B component of the coord
    pub b: Option<Value>,
    /// The optional C component of the coord
    pub c: Option<Value>,
    /// The optional U component of the coord
    pub u: Option<Value>,
    /// The optional V component of the coord
    pub v: Option<Value>,
    /// The optional W component of the coord
    pub w: Option<Value>,
}

impl Default for Coord {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            a: None,
            b: None,
            c: None,
            u: None,
            v: None,
            w: None,
        }
    }
}

static EMPTY_COORD: Coord = Coord {
    x: None,
    y: None,
    z: None,
    a: None,
    b: None,
    c: None,
    u: None,
    v: None,
    w: None,
};

/// Parse a coordinate
pub fn coord<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Coord, E> {
    context(
        "coordinate",
        map_opt(
            permutation((
                opt(preceded_value(tag_no_case("X"))),
                opt(preceded_value(tag_no_case("Y"))),
                opt(preceded_value(tag_no_case("Z"))),
                opt(preceded_value(tag_no_case("A"))),
                opt(preceded_value(tag_no_case("B"))),
                opt(preceded_value(tag_no_case("C"))),
                opt(preceded_value(tag_no_case("U"))),
                opt(preceded_value(tag_no_case("V"))),
                opt(preceded_value(tag_no_case("W"))),
            )),
            |(x, y, z, a, b, c, u, v, w)| {
                let coord = Coord {
                    x,
                    y,
                    z,
                    a,
                    b,
                    c,
                    u,
                    v,
                    w,
                };

                if coord == EMPTY_COORD {
                    None
                } else {
                    Some(coord)
                }
            },
        ),
    )(i)
}

// named_attr!(#[doc = "Parse a coordinate"], pub coord<Span, Coord>,
//     map_opt!(
//         sep!(
//             space0,
//             permutation!(
//                 preceded!(char_no_case!('X'), ngc_float_value)?,
//                 preceded!(char_no_case!('Y'), ngc_float_value)?,
//                 preceded!(char_no_case!('Z'), ngc_float_value)?,
//                 preceded!(char_no_case!('A'), ngc_float_value)?,
//                 preceded!(char_no_case!('B'), ngc_float_value)?,
//                 preceded!(char_no_case!('C'), ngc_float_value)?,
//                 preceded!(char_no_case!('U'), ngc_float_value)?,
//                 preceded!(char_no_case!('V'), ngc_float_value)?,
//                 preceded!(char_no_case!('W'), ngc_float_value)?
//             )
//         ),
//         |(x, y, z, a, b, c, u, v, w)| {
//             let coord = Coord { x, y, z, a, b, c, u, v, w };

//             if coord == EMPTY_COORD {
//                 None
//             } else {
//                 Some(coord)
//             }
//         }
//     )
// );

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;

    #[test]
    fn parse_xyz() {
        assert_parse!(
            parser = coord;
            input = "X0.0 Y1.0 Z2.0";
            expected = coord!(0.0, 1.0, 2.0)
        );
    }

    #[test]
    fn parse_lowercase() {
        assert_parse!(
            parser = coord;
            input = "x0.0 y1.0 z2.0";
            expected = coord!(0.0, 1.0, 2.0)
        );
    }

    #[test]
    fn parse_lowercase_integers() {
        assert_parse!(
            parser = coord;
            input = "x12 y34 z56";
            expected = coord!(12.0, 34.0, 56.0)
        );
    }

    #[test]
    fn parse_no_whitespace() {
        assert_parse!(
            parser = coord;
            input = "x12y34z56";
            expected = coord!(12.0, 34.0, 56.0)
        );
    }

    #[test]
    fn parse_real_world() {
        assert_parse!(
            parser = coord;
            input = "X0 Y0 z 20";
            expected = coord!(0.0, 0.0, 20.0)
        );

        assert_parse!(
            parser = coord;
            input = "Z5.";
            expected = Coord {
                z: Some(5.0.into()),
                ..Coord::default()
            }
        );
    }
}
