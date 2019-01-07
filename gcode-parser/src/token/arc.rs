use common::parsing::Span;
use expression::{
    parser::{ngc_float_value, ngc_unsigned_value},
    Value,
};
use nom::*;

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
    /// Defaults to `0`, meaning no full turns are made
    pub turns: Value,
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
    /// Defaults to `0`, meaning no full turns are made
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
            turns: Value::Unsigned(0),
        }
    }
}

named_attr!(#[doc = "Parse a center format arc"], pub center_format_arc<Span, CenterFormatArc>,
    map_opt!(
        sep!(
            space0,
            permutation!(
                preceded!(char_no_case!('X'), ngc_float_value)?,
                preceded!(char_no_case!('Y'), ngc_float_value)?,
                preceded!(char_no_case!('Z'), ngc_float_value)?,
                preceded!(char_no_case!('I'), ngc_float_value)?,
                preceded!(char_no_case!('J'), ngc_float_value)?,
                preceded!(char_no_case!('K'), ngc_float_value)?,
                preceded!(char_no_case!('P'), ngc_unsigned_value)?
            )
        ),
        |(x, y, z, i, j, k, turns): (Option<Value>, Option<Value>, Option<Value>, Option<Value>, Option<Value>, Option<Value>, Option<Value>)| {
            let arc = CenterFormatArc { x, y, z, i, j, k, turns: turns.unwrap_or(Value::Unsigned(0)) };

            // TODO: Validate actual valid combinations of these coords as per [the docs](http://linuxcnc.org/docs/html/gcode/g-code.html#gcode:g2-g3)
            // TODO: Return validation error instead of `None`
            if (&arc.x, &arc.y, &arc.z) == (&None, &None, &None) || (&arc.i, &arc.j, &arc.k) == (&None, &None, &None) {
                None
            } else {
                Some(arc)
            }
        }
    )
);

named_attr!(#[doc = "Parse a radius format arc"], pub radius_format_arc<Span, RadiusFormatArc>,
    map_opt!(
        sep!(
            space0,
            permutation!(
                preceded!(char_no_case!('X'), ngc_float_value)?,
                preceded!(char_no_case!('Y'), ngc_float_value)?,
                preceded!(char_no_case!('Z'), ngc_float_value)?,
                preceded!(char_no_case!('R'), ngc_float_value),
                preceded!(char_no_case!('P'), ngc_unsigned_value)?
            )
        ),
        |(x, y, z, radius, turns): (Option<Value>, Option<Value>, Option<Value>, Value, Option<Value>)| {
            let arc = RadiusFormatArc { x, y, z, radius, turns: turns.unwrap_or(Value::Unsigned(0)) };

            // TODO: Validate actual valid combinations of these coords as per [the docs](http://linuxcnc.org/docs/html/gcode/g-code.html#gcode:g2-g3)
            // TODO: Return validation error instead of `None`
            if (&arc.x, &arc.y, &arc.z) == (&None, &None, &None) {
                None
            } else {
                Some(arc)
            }
        }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};

    #[test]
    fn parse_center_format_arc() {
        assert_parse!(
            parser = center_format_arc;
            input = span!(b"X0 Y1 I2 J3");
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
            input = span!(b"X0 Y1 I2 J3 P5");
            expected = CenterFormatArc {
                x: Some(0.0f32.into()),
                y: Some(1.0f32.into()),
                i: Some(2.0f32.into()),
                j: Some(3.0f32.into()),
                turns: Value::Unsigned(5),
                ..CenterFormatArc::default()
            }
        );
    }

    #[test]
    fn arc_real_world() {
        assert_parse!(
            parser = center_format_arc;
            input = span!(b"X0 Y0 z 20 I20 J0");
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
            input = span!(b"X-2.4438 Y-0.2048 I-0.0766 J0.2022");
            expected = CenterFormatArc {
                x: Some((-2.4438f32).into()),
                y: Some((-0.2048f32).into()),
                i: Some((-0.0766f32).into()),
                j: Some(0.2022f32.into()),
                ..CenterFormatArc::default()
            }
        );
    }

    #[test]
    fn backwards_center_format() {
        assert_parse!(
            parser = center_format_arc;
            input = span!(b"I-2.070552 J-7.727407 X36.817108 Y-8.797959 Z-3.500000");
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
}
