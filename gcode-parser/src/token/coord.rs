//! Parse coordinates into a vector

use crate::value::decimal_value;
use crate::value::Value;
use nom::character::complete::anychar;
use nom::error::ErrorKind;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::Err;
use nom::{character::complete::space0, error::ParseError, IResult};

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

/// Parse a coordinate
pub fn coord<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Coord, E> {
    let mut c = Coord::default();
    let mut input = i;

    for _ in 0..9 {
        let res = preceded(space0, separated_pair(anychar, space0, decimal_value))(input);

        match res {
            Ok((i, (ch, value))) => {
                match ch.to_ascii_lowercase() {
                    'x' if c.x.is_none() => {
                        c.x = Some(value);
                        input = i;
                    }
                    'y' if c.y.is_none() => {
                        c.y = Some(value);
                        input = i;
                    }
                    'z' if c.z.is_none() => {
                        c.z = Some(value);
                        input = i;
                    }
                    //
                    'a' if c.a.is_none() => {
                        c.a = Some(value);
                        input = i;
                    }
                    'b' if c.b.is_none() => {
                        c.b = Some(value);
                        input = i;
                    }
                    'c' if c.c.is_none() => {
                        c.c = Some(value);
                        input = i;
                    }
                    //
                    'u' if c.u.is_none() => {
                        c.u = Some(value);
                        input = i;
                    }
                    'v' if c.v.is_none() => {
                        c.v = Some(value);
                        input = i;
                    }
                    'w' if c.w.is_none() => {
                        c.w = Some(value);
                        input = i;
                    }
                    // ---
                    'x' if c.x.is_some() => break,
                    'y' if c.y.is_some() => break,
                    'z' if c.z.is_some() => break,
                    //
                    'a' if c.a.is_some() => break,
                    'b' if c.b.is_some() => break,
                    'c' if c.c.is_some() => break,
                    //
                    'u' if c.u.is_some() => break,
                    'v' if c.v.is_some() => break,
                    'w' if c.w.is_some() => break,

                    _ => (),
                }
            }
            Err(Err::Error(e)) => {
                if c == Coord::default() {
                    return Err(Err::Error(E::append(input, ErrorKind::ManyMN, e)));
                } else {
                    return Ok((input, c));
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    if c != Coord::default() {
        Ok((input, c))
    } else {
        Err(Err::Error(E::from_error_kind(input, ErrorKind::ManyMN)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use expression::Parameter;

    #[test]
    fn parse_xxyz() {
        assert_parse!(
            parser = coord;
            input = "X0.0 X3.0 Y1.0 Z2.0";
            expected = coord!(0.0);
            remaining = " X3.0 Y1.0 Z2.0"
        );
    }

    #[test]
    fn parse_var() {
        assert_parse!(
            parser = coord;
            input = "X#3";
            expected = Coord {
                x: Some(Parameter::Numbered(3).into()),
                ..Coord::default()
            }
        );
    }

    #[test]
    fn parse_xyz() {
        assert_parse!(
            parser = coord;
            input = "X0.0 Y1.0 Z2.0";
            expected = coord!(0.0, 1.0, 2.0)
        );
    }

    #[test]
    fn parse_xyz_integer() {
        assert_parse!(
            parser = coord;
            input = "X0 Y1 Z2";
            expected = coord!(0.0, 1.0, 2.0)
        );
    }

    #[test]
    fn parse_wbx() {
        assert_parse!(
            parser = coord;
            input = "W0.0 B1.0 X2.0";
            expected = Coord {
                w: Some(0.0.into()),
                b: Some(1.0.into()),
                x: Some(2.0.into()),
                ..Coord::default()
            }
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
