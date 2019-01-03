//! Parse coordinates into a vector

use crate::parsers::ngc_float;
use common::parsing::Span;
use nom::*;

/// A 9 dimensional `XYZABCUVW` coordinate
///
/// Any or all of the components of this coordinate can be `None`, however the parser requires at
/// least _one_ populated field to parse successfully.
#[derive(Debug, PartialEq, Clone)]
pub struct Coord {
    /// The optional X component of the coord
    pub x: Option<f32>,
    /// The optional Y component of the coord
    pub y: Option<f32>,
    /// The optional Z component of the coord
    pub z: Option<f32>,
    /// The optional A component of the coord
    pub a: Option<f32>,
    /// The optional B component of the coord
    pub b: Option<f32>,
    /// The optional C component of the coord
    pub c: Option<f32>,
    /// The optional U component of the coord
    pub u: Option<f32>,
    /// The optional V component of the coord
    pub v: Option<f32>,
    /// The optional W component of the coord
    pub w: Option<f32>,
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

named_attr!(#[doc = "Parse a coordinate"], pub coord<Span, Coord>,
    map_opt!(
        sep!(
            space0,
            permutation!(
                sep!(space0, preceded!(char_no_case!('X'), ngc_float))?,
                sep!(space0, preceded!(char_no_case!('Y'), ngc_float))?,
                sep!(space0, preceded!(char_no_case!('Z'), ngc_float))?,
                sep!(space0, preceded!(char_no_case!('A'), ngc_float))?,
                sep!(space0, preceded!(char_no_case!('B'), ngc_float))?,
                sep!(space0, preceded!(char_no_case!('C'), ngc_float))?,
                sep!(space0, preceded!(char_no_case!('U'), ngc_float))?,
                sep!(space0, preceded!(char_no_case!('V'), ngc_float))?,
                sep!(space0, preceded!(char_no_case!('W'), ngc_float))?
            )
        ),
        |(x, y, z, a, b, c, u, v, w)| {
            let coord = Coord { x, y, z, a, b, c, u, v, w };

            if coord == EMPTY_COORD {
                None
            } else {
                Some(coord)
            }
        }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_xyz() {
        assert_parse!(
            parser = coord;
            input = span!(b"X0.0 Y1.0 Z2.0");
            expected = Coord {
                x: Some(0.0),
                y: Some(1.0),
                z: Some(2.0),
                ..Coord::default()
            }
        );
    }

    #[test]
    fn parse_lowercase() {
        assert_parse!(
            parser = coord;
            input = span!(b"x0.0 y1.0 z2.0");
            expected = Coord {
                x: Some(0.0),
                y: Some(1.0),
                z: Some(2.0),
                ..Coord::default()
            }
        );
    }

    #[test]
    fn parse_lowercase_integers() {
        assert_parse!(
            parser = coord;
            input = span!(b"x12 y34 z56");
            expected = Coord {
                x: Some(12.0),
                y: Some(34.0),
                z: Some(56.0),
                ..Coord::default()
            }
        );
    }

    #[test]
    fn parse_no_whitespace() {
        assert_parse!(
            parser = coord;
            input = span!(b"x12y34z56");
            expected = Coord {
                x: Some(12.0),
                y: Some(34.0),
                z: Some(56.0),
                ..Coord::default()
            }
        );
    }

    #[test]
    fn parse_real_world() {
        assert_parse!(
            parser = coord;
            input = span!(b"X0 Y0 z 20");
            expected = coord!(0.0, 0.0, 20.0)
        );

        assert_parse!(
            parser = coord;
            input = span!(b"Z5.");
            expected = Coord {
                z: Some(5.0),
                ..Coord::default()
            }
        );
    }
}
