use crate::parsers::char_no_case;
use crate::value::{preceded_decimal_value, preceded_unsigned_value, UnsignedValue, Value};
use nom::{
    branch::permutation,
    character::complete::space0,
    combinator::{map_opt, map_res, opt},
    error::{context, ParseError},
    sequence::terminated,
    IResult,
};

/// Center format arc offsets
///
/// TODO: The parser does not currently validate that the offset/axis combinations are valid.
#[derive(Debug, PartialEq, Clone)]
pub struct CenterFormatArc {
    /// Arc end position, X component
    pub x: Option<Value>,
    /// Arc end position, Y component
    pub y: Option<Value>,
    /// Arc end position, Z component
    pub z: Option<Value>,
    /// Arc offset
    pub i: Option<Value>,
    /// Arc offset
    pub j: Option<Value>,
    /// Arc offset
    pub k: Option<Value>,
    /// Number of turns
    ///
    /// Defaults to `1`, a full circle
    pub turns: UnsignedValue,
}

/// Radius format arc
///
/// TODO: The parser does not currently validate that the offset/axis combinations are valid.
#[derive(Debug, PartialEq, Clone)]
pub struct RadiusFormatArc {
    /// Arc end position, X component
    pub x: Option<Value>,
    /// Arc end position, Y component
    pub y: Option<Value>,
    /// Arc end position, Z component
    pub z: Option<Value>,
    /// Arc radius
    pub radius: Value,
    /// Number of turns
    ///
    /// Defaults to `1`
    pub turns: Value,
}

impl Default for CenterFormatArc {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            i: None,
            j: None,
            k: None,
            turns: 1.into(),
        }
    }
}

// named_attr!(#[doc = "Parse a center format arc"], pub center_format_arc<Span, CenterFormatArc>,
//     map_opt!(
//         sep!(
//             space0,
//             permutation!(
//                 preceded!(char_no_case!('X'), ngc_float_value)?,
//                 preceded!(char_no_case!('Y'), ngc_float_value)?,
//                 preceded!(char_no_case!('Z'), ngc_float_value)?,
//                 preceded!(char_no_case!('I'), ngc_float_value)?,
//                 preceded!(char_no_case!('J'), ngc_float_value)?,
//                 preceded!(char_no_case!('K'), ngc_float_value)?,
//                 preceded!(char_no_case!('P'), ngc_unsigned_value)?
//             )
//         ),
//         |(x, y, z, i, j, k, turns): (Option<Value>, Option<Value>, Option<Value>, Option<Value>, Option<Value>, Option<Value>, Option<Value>)| {
//             let arc = CenterFormatArc { x, y, z, i, j, k, turns: turns.unwrap_or(Value::Unsigned(1)) };

//             // TODO: Validate actual valid combinations of these coords as per [the docs](http://linuxcnc.org/docs/html/gcode/g-code.html#gcode:g2-g3)
//             // TODO: Return validation error instead of `None`
//             // Require at least one offset coordinate to be present
//             if (&arc.i, &arc.j, &arc.k) == (&None, &None, &None) {
//                 None
//             } else {
//                 Some(arc)
//             }
//         }
//     )
// );

/// Parse a center format arc
pub fn center_format_arc<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, CenterFormatArc, E> {
    context(
        "center format arc",
        map_opt(
            permutation((
                opt(terminated(
                    preceded_decimal_value(char_no_case('X')),
                    space0,
                )),
                opt(terminated(
                    preceded_decimal_value(char_no_case('Y')),
                    space0,
                )),
                opt(terminated(
                    preceded_decimal_value(char_no_case('Z')),
                    space0,
                )),
                opt(terminated(
                    preceded_decimal_value(char_no_case('I')),
                    space0,
                )),
                opt(terminated(
                    preceded_decimal_value(char_no_case('J')),
                    space0,
                )),
                opt(terminated(
                    preceded_decimal_value(char_no_case('K')),
                    space0,
                )),
                // TODO: This must be a positive integer, not any `Value`
                opt(terminated(
                    preceded_unsigned_value(char_no_case('P')),
                    space0,
                )),
            )),
            |(x, y, z, i, j, k, turns): (
                Option<Value>,
                Option<Value>,
                Option<Value>,
                Option<Value>,
                Option<Value>,
                Option<Value>,
                Option<UnsignedValue>,
            )| {
                let arc = CenterFormatArc {
                    x,
                    y,
                    z,
                    i,
                    j,
                    k,
                    // TODO: Parse into integer
                    turns: turns.unwrap_or(1.into()),
                };

                // TODO: Validate actual valid combinations of these coords as per [the docs](http://linuxcnc.org/docs/html/gcode/g-code.html#gcode:g2-g3)
                // TODO: Return validation error instead of `None`
                // Require at least one offset coordinate to be present
                if (&arc.i, &arc.j, &arc.k) == (&None, &None, &None) {
                    // Err("Invalid center format arc")
                    None
                } else {
                    Some(arc)
                }
            },
        ),
    )(i)
}

// named_attr!(#[doc = "Parse a radius format arc"], pub radius_format_arc<Span, RadiusFormatArc>,
//     map_opt!(
//         sep!(
//             space0,
//             permutation!(
//                 preceded!(char_no_case!('X'), ngc_float_value)?,
//                 preceded!(char_no_case!('Y'), ngc_float_value)?,
//                 preceded!(char_no_case!('Z'), ngc_float_value)?,
//                 preceded!(char_no_case!('R'), ngc_float_value),
//                 preceded!(char_no_case!('P'), ngc_unsigned_value)?
//             )
//         ),
//         |(x, y, z, radius, turns): (Option<Value>, Option<Value>, Option<Value>, Value, Option<Value>)| {
//             let arc = RadiusFormatArc { x, y, z, radius, turns: turns.unwrap_or(Value::Unsigned(1)) };

//             // TODO: Validate actual valid combinations of these coords as per [the docs](http://linuxcnc.org/docs/html/gcode/g-code.html#gcode:g2-g3)
//             // TODO: Return validation error instead of `None`
//             if (&arc.x, &arc.y, &arc.z) == (&None, &None, &None) {
//                 None
//             } else {
//                 Some(arc)
//             }
//         }
//     )
// );

/// Parse a radius format arc
pub fn radius_format_arc<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, RadiusFormatArc, E> {
    context(
        "radius format arc",
        map_res(
            // TODO: Needs some sort of sep() to handle whitespace
            permutation((
                opt(terminated(
                    preceded_decimal_value(char_no_case('X')),
                    space0,
                )),
                opt(terminated(
                    preceded_decimal_value(char_no_case('Y')),
                    space0,
                )),
                opt(terminated(
                    preceded_decimal_value(char_no_case('Z')),
                    space0,
                )),
                terminated(preceded_decimal_value(char_no_case('R')), space0),
                // TODO: This must be a positive integer, not any `Value`
                opt(preceded_decimal_value(char_no_case('P'))),
            )),
            |(x, y, z, radius, _turns): (
                Option<Value>,
                Option<Value>,
                Option<Value>,
                Value,
                Option<Value>,
            )| {
                let arc = RadiusFormatArc {
                    x,
                    y,
                    z,
                    radius,
                    // TODO: Use parsed value as positive integer
                    // turns: turns.unwrap_or(Value::Unsigned(1)),
                    turns: 1.0.into(),
                };

                // TODO: Validate actual valid combinations of these coords as per [the docs](http://linuxcnc.org/docs/html/gcode/g-code.html#gcode:g2-g3)
                // TODO: Return validation error instead of `None`
                if (&arc.x, &arc.y, &arc.z) == (&None, &None, &None) {
                    Err("Invalid radius format")
                } else {
                    Ok(arc)
                }
            },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;

    #[test]
    fn parse_center_format_arc() {
        assert_parse!(
            parser = center_format_arc;
            input = "X0 Y1 I2 J3";
            expected = CenterFormatArc {
                x: Some(0.0f32.into()),
                y: Some(1.0f32.into()),
                i: Some(2.0f32.into()),
                j: Some(3.0f32.into()),
                ..CenterFormatArc::default()
            }
        );
    }

    #[test]
    fn center_format_arc_num_turns() {
        assert_parse!(
            parser = center_format_arc;
            input = "X0 Y1 I2 J3 P5";
            expected = CenterFormatArc {
                x: Some(0.0f32.into()),
                y: Some(1.0f32.into()),
                i: Some(2.0f32.into()),
                j: Some(3.0f32.into()),
                turns: 5.into(),
                ..CenterFormatArc::default()
            }
        );
    }

    #[test]
    fn arc_real_world() {
        assert_parse!(
            parser = center_format_arc;
            input = "X0 Y0 z 20 I20 J0";
            expected = CenterFormatArc {
                x: Some(0.0f32.into()),
                y: Some(0.0f32.into()),
                z: Some(20.0f32.into()),
                i: Some(20.0f32.into()),
                j: Some(0.0f32.into()),
                ..CenterFormatArc::default()
            }
        );

        assert_parse!(
            parser = center_format_arc;
            input = "X-2.4438 Y-0.2048 I-0.0766 J0.2022";
            expected = CenterFormatArc {
                x: Some((-2.4438f32).into()),
                y: Some((-0.2048f32).into()),
                i: Some((-0.0766f32).into()),
                j: Some(0.2022f32.into()),
                ..CenterFormatArc::default()
            }
        );
    }

    // TODO: Re-enable once a solution is found for <https://github.com/Geal/nom/issues/988>
    #[test]
    #[ignore]
    fn backwards_center_format() {
        assert_parse!(
            parser = center_format_arc;
            input = "I-2.070552 J-7.727407 X36.817108 Y-8.797959 Z-3.500000";
            expected = CenterFormatArc {
                x: Some(36.817108f32.into()),
                y: Some((-8.797959f32).into()),
                z: Some((-3.500000f32).into()),
                i: Some((-2.070552f32).into()),
                j: Some((-7.727407f32).into()),
                ..CenterFormatArc::default()
            }
        );
    }

    #[test]
    fn full_circle() {
        assert_parse!(
            parser = center_format_arc;
            input = "I -20";
            expected = CenterFormatArc {
                i: Some((-20.0f32).into()),
                ..CenterFormatArc::default()
            }
        );
    }
}
