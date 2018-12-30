use crate::Span;
use nom::*;

/// Center format arc offsets
///
/// TODO: The parser does not currently validate that the offset/axis combinations are valid.
#[derive(Debug, PartialEq, Clone)]
pub struct CenterFormatArc {
    /// Arc end position, X component
    pub x: Option<f32>,
    /// Arc end position, Y component
    pub y: Option<f32>,
    /// Arc end position, Z component
    pub z: Option<f32>,
    /// Arc offset
    pub i: Option<f32>,
    /// Arc offset
    pub k: Option<f32>,
    /// Arc offset
    pub j: Option<f32>,
    /// Number of turns
    ///
    /// Defaults to `0`, meaning no full turns are made
    pub turns: u32,
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
            turns: 0,
        }
    }
}

named!(pub center_format_arc<Span, CenterFormatArc>,
    map_opt!(
        sep!(
            space0,
            permutation!(
                opt!(sep!(space0, preceded!(char_no_case!('X'), float))),
                opt!(sep!(space0, preceded!(char_no_case!('Y'), float))),
                opt!(sep!(space0, preceded!(char_no_case!('Z'), float))),
                opt!(sep!(space0, preceded!(char_no_case!('I'), float))),
                opt!(sep!(space0, preceded!(char_no_case!('J'), float))),
                opt!(sep!(space0, preceded!(char_no_case!('K'), float))),
                opt!(sep!(space0, preceded!(char_no_case!('P'), flat_map!(digit1, parse_to!(u32)))))
            )
        ),
        |(x, y, z, i, j, k, turns): (Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<u32>)| {
            let arc = CenterFormatArc { x, y, z, i, j, k, turns: turns.unwrap_or(0) };

            // TODO: Validate actual valid combinations of these coords as per [the docs](http://linuxcnc.org/docs/html/gcode/g-code.html#gcode:g2-g3)
            if (x, y, z) == (None, None, None) || (i, j, k) == (None, None, None) {
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

    #[test]
    fn parse_center_format_arc() {
        assert_parse!(
            parser = center_format_arc,
            input = span!(b"X0 Y1 I2 J3"),
            expected = CenterFormatArc {
                x: Some(0.0),
                y: Some(1.0),
                i: Some(2.0),
                j: Some(3.0),
                ..CenterFormatArc::default()
            }
        );
    }

    #[test]
    fn center_format_arc_num_turns() {
        assert_parse!(
            parser = center_format_arc,
            input = span!(b"X0 Y1 I2 J3 P5"),
            expected = CenterFormatArc {
                x: Some(0.0),
                y: Some(1.0),
                i: Some(2.0),
                j: Some(3.0),
                turns: 5,
                ..CenterFormatArc::default()
            }
        );
    }

    #[test]
    fn arc_real_world() {
        assert_parse!(
            parser = center_format_arc,
            input = span!(b"X0 Y0 z 20 I20 J0"),
            expected = CenterFormatArc {
                x: Some(0.0),
                y: Some(0.0),
                z: Some(20.0),
                i: Some(20.0),
                j: Some(0.0),
                ..CenterFormatArc::default()
            }
        );

        assert_parse!(
            parser = center_format_arc,
            input = span!(b"X-2.4438 Y-0.2048 I-0.0766 J0.2022"),
            expected = CenterFormatArc {
                x: Some(-2.4438),
                y: Some(-0.2048),
                i: Some(-0.0766),
                j: Some(0.2022),
                ..CenterFormatArc::default()
            }
        );
    }
}
