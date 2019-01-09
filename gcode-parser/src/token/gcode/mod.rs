mod cutter_compensation;
mod dwell;
mod plane_select;
mod work_offset;

use self::cutter_compensation::cutter_compensation;
pub use self::cutter_compensation::CutterCompensation;
use self::dwell::dwell;
pub use self::dwell::Dwell;
use self::plane_select::plane_select;
pub use self::plane_select::PlaneSelect;
use self::work_offset::work_offset;
pub use self::work_offset::{WorkOffset, WorkOffsetValue};
use crate::map_code;
use common::parsing::Span;
use nom::*;

/// A G-code
#[derive(Debug, PartialEq, Clone)]
pub enum GCode {
    /// Rapid move
    Rapid,

    /// Move at a defined feedrate
    Feed,

    /// Work offset (`G54`, `G55`, etc)
    WorkOffset(WorkOffset),

    /// Wait for a (decimal) number of seconds
    Dwell(Dwell),

    /// Set units to metric (millimeters)
    UnitsMM,

    /// Set units to imperial (inch)
    UnitsInch,

    /// Plane select (XY, UV, etc)
    PlaneSelect(PlaneSelect),

    /// A clockwise arc
    ClockwiseArc,

    /// A counterclockwise arc
    CounterclockwiseArc,

    /// Disable cutter compensation (G40)
    DisableCutterCompensation,

    /// Cutter compensation (off, left, right)
    CutterCompensation(CutterCompensation),
}

named!(pub gcode<Span, GCode>,
    alt!(
        map_code!("G0", |_| GCode::Rapid) |
        map_code!("G1", |_| GCode::Feed) |
        map_code!("G2", |_| GCode::ClockwiseArc) |
        map_code!("G3", |_| GCode::CounterclockwiseArc) |
        map_code!("G21", |_| GCode::UnitsMM) |
        map_code!("G20", |_| GCode::UnitsInch) |
        map!(work_offset, GCode::WorkOffset) |
        map!(cutter_compensation, GCode::CutterCompensation) |
        map!(plane_select, GCode::PlaneSelect) |
        map!(dwell, GCode::Dwell)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};

    #[test]
    fn parse_rapid() {
        assert_parse!(
            parser = gcode;
            input = span!(b"G0");
            expected = GCode::Rapid
        );

        assert_parse!(
            parser = gcode;
            input = span!(b"G00");
            expected = GCode::Rapid
        );
    }

    #[test]
    fn parse_feed() {
        assert_parse!(parser = gcode; input = span!(b"G1"); expected = GCode::Feed);

        assert_parse!(parser = gcode; input = span!(b"G01"); expected = GCode::Feed);
    }

    #[test]
    fn parse_arc() {
        assert_parse!(
            parser = gcode;
            input = span!(b"G2");
            expected = GCode::ClockwiseArc
        );

        assert_parse!(
            parser = gcode;
            input = span!(b"G3");
            expected = GCode::CounterclockwiseArc
        );
    }
}
