//! Parse polar coordinates

use crate::parsers::char_no_case;
use crate::value::{preceded_decimal_value, Value};
use nom::{
    branch::permutation,
    character::complete::space0,
    combinator::{map_res, opt},
    error::{context, ParseError},
    sequence::terminated,
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

impl Default for PolarCoord {
    fn default() -> Self {
        Self {
            distance: None,
            angle: None,
        }
    }
}

pub fn polar<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PolarCoord, E> {
    context(
        "polar coordinate",
        map_res(
            permutation((
                opt(terminated(
                    preceded_decimal_value(char_no_case('@')),
                    space0,
                )),
                opt(preceded_decimal_value(char_no_case('^'))),
            )),
            |(distance, angle)| {
                let res = PolarCoord { distance, angle };

                if res != PolarCoord::default() {
                    Ok(res)
                } else {
                    Err("polar coordinate may not be empty")
                }
            },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use nom::error::VerboseError;

    #[test]
    fn parse_polar_empty() {
        let res = polar::<VerboseError<&str>>("");

        assert!(res.is_err());
    }

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
