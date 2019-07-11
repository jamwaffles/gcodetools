//! Parse coordinates into a vector

use crate::parsers::char_no_case;
use crate::value::{preceded_decimal_value, Value};
use nom::{
    branch::alt, character::complete::space0, combinator::map, error::ParseError, multi::many_m_n,
    sequence::terminated, IResult,
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

#[derive(Debug, PartialEq, Clone)]
enum CoordPart {
    X(Value),
    Y(Value),
    Z(Value),
    A(Value),
    B(Value),
    C(Value),
    U(Value),
    V(Value),
    W(Value),
}

/// Parse a coordinate
///
/// TODO: Fail when more than one of each axis is encountered
pub fn coord<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Coord, E> {
    map(
        many_m_n(
            1,
            9,
            terminated(
                alt((
                    map(preceded_decimal_value(char_no_case('X')), CoordPart::X),
                    map(preceded_decimal_value(char_no_case('Y')), CoordPart::Y),
                    map(preceded_decimal_value(char_no_case('Z')), CoordPart::Z),
                    map(preceded_decimal_value(char_no_case('A')), CoordPart::A),
                    map(preceded_decimal_value(char_no_case('B')), CoordPart::B),
                    map(preceded_decimal_value(char_no_case('C')), CoordPart::C),
                    map(preceded_decimal_value(char_no_case('U')), CoordPart::U),
                    map(preceded_decimal_value(char_no_case('V')), CoordPart::V),
                    map(preceded_decimal_value(char_no_case('W')), CoordPart::W),
                )),
                space0,
            ),
        ),
        |parts| {
            parts.into_iter().fold(Coord::default(), |mut carry, part| {
                match part {
                    CoordPart::X(p) => carry.x = Some(p),
                    CoordPart::Y(p) => carry.y = Some(p),
                    CoordPart::Z(p) => carry.z = Some(p),
                    CoordPart::A(p) => carry.a = Some(p),
                    CoordPart::B(p) => carry.b = Some(p),
                    CoordPart::C(p) => carry.c = Some(p),
                    CoordPart::U(p) => carry.u = Some(p),
                    CoordPart::V(p) => carry.v = Some(p),
                    CoordPart::W(p) => carry.w = Some(p),
                };

                carry
            })
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use expression::Parameter;

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
