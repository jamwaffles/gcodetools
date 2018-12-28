use crate::token::{coord, Coord};
use crate::{map_code, Span};
use nom::*;

/// The type of arc definition
#[derive(Debug, PartialEq, Clone)]
pub enum ArcFormat {
    /// Center format arc
    Center,

    /// Radius format arc
    Radius,
}

/// Arc direction as viewed from the positive end of its axis of rotation
#[derive(Debug, PartialEq, Clone)]
pub enum ArcDirection {
    /// Clockwise (G2)
    Clockwise,

    /// Counterclockwise (G3)
    Counterclockwise,
}

/// Arc offsets
#[derive(Debug, PartialEq, Clone)]
pub struct ArcOffset {
    i: Option<f32>,
    k: Option<f32>,
    j: Option<f32>,
}

impl Default for ArcOffset {
    fn default() -> Self {
        Self {
            i: None,
            j: None,
            k: None,
        }
    }
}

/// Clockwise arc (G2)
#[derive(Debug, PartialEq, Clone)]
pub struct Arc {
    /// Arc direction (CW or CCW)
    direction: ArcDirection,

    /// The format this arc is defined with
    format: ArcFormat,

    /// The end point and helix angle of the arc
    position_data: Coord,

    /// Number of turns
    turns: u32,

    /// Offsets
    offset: ArcOffset,
}

named!(arc_offset<Span, ArcOffset>,
    map_res!(
        sep!(
            space0,
            permutation!(
                opt!(sep!(space0, preceded!(char_no_case!('I'), float))),
                opt!(sep!(space0, preceded!(char_no_case!('J'), float))),
                opt!(sep!(space0, preceded!(char_no_case!('K'), float)))
            )
        ),
        |(i, j, k)| {
            let offs = ArcOffset { i, j, k };

            if offs == ArcOffset::default() {
                Err(())
            } else {
                Ok(offs)
            }
        }
    )
);

// TODO: Only allow valid combinations of XYZIJK as per
// [the docs](http://linuxcnc.org/docs/html/gcode/g-code.html#gcode:g2-g3)
named!(pub center_format_clockwise_arc<Span, Arc>,
    map_code!(
        "G2",
        tuple!(
            coord,
            arc_offset,
            opt!(
                flat_map!(
                    preceded!(char_no_case!('P'), digit1),
                    parse_to!(u32)
                )
            )
        ),
        |(position_data, offset, turns)| {
            Arc {
                direction: ArcDirection::Clockwise,
                format: ArcFormat::Center,
                position_data,
                turns: turns.unwrap_or(0),
                offset
            }
        }
    )
);

// TODO: DRY with G2 parse
named!(pub center_format_counterclockwise_arc<Span, Arc>,
    map_code!(
        "G3",
        tuple!(
            coord,
            arc_offset,
            opt!(
                flat_map!(
                    preceded!(char_no_case!('P'), digit1),
                    parse_to!(u32)
                )
            )
        ),
        |(position_data, offset, turns)| {
            Arc {
                direction: ArcDirection::Counterclockwise,
                format: ArcFormat::Center,
                position_data,
                turns: turns.unwrap_or(0),
                offset
            }
        }
    )
);

named!(pub arc<Span, Arc>,
    alt_complete!(
        center_format_clockwise_arc |
        center_format_counterclockwise_arc
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_format_arc() {
        assert_parse!(
            parser = arc,
            input = span!(b"G2 X0 Y1 I2 J3"),
            expected = Arc {
                direction: ArcDirection::Clockwise,
                format: ArcFormat::Center,
                position_data: coord!(0.0, 1.0),
                offset: ArcOffset {
                    i: Some(2.0),
                    j: Some(3.0),
                    ..ArcOffset::default()
                },
                turns: 0u32
            }
        );

        assert_parse!(
            parser = arc,
            input = span!(b"G3 X0 Y1 I2 J3"),
            expected = Arc {
                direction: ArcDirection::Counterclockwise,
                format: ArcFormat::Center,
                position_data: coord!(0.0, 1.0),
                offset: ArcOffset {
                    i: Some(2.0),
                    j: Some(3.0),
                    ..ArcOffset::default()
                },
                turns: 0u32
            }
        );
    }

    #[test]
    fn center_format_arc_num_turns() {
        assert_parse!(
            parser = arc,
            input = span!(b"G2 X0 Y1 I2 J3 P5"),
            expected = Arc {
                direction: ArcDirection::Clockwise,
                format: ArcFormat::Center,
                position_data: coord!(0.0, 1.0),
                offset: ArcOffset {
                    i: Some(2.0),
                    j: Some(3.0),
                    ..ArcOffset::default()
                },
                turns: 5u32
            }
        );
    }

    #[test]
    fn arc_real_world() {
        assert_parse!(
            parser = arc,
            input = span!(b"G3 X0 Y0 z 20 I20 J0"),
            expected = Arc {
                direction: ArcDirection::Counterclockwise,
                format: ArcFormat::Center,
                position_data: coord!(0.0, 0.0, 20.0),
                offset: ArcOffset {
                    i: Some(20.0),
                    j: Some(0.0),
                    ..ArcOffset::default()
                },
                turns: 0u32
            }
        );
    }
}
