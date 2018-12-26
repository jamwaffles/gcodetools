mod dwell;
mod plane_select;
mod work_offset;

use self::dwell::dwell;
pub use self::dwell::Dwell;
use self::plane_select::plane_select;
pub use self::plane_select::PlaneSelect;
use self::work_offset::work_offset;
pub use self::work_offset::{WorkOffset, WorkOffsetValue};
use crate::Span;
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
}

named!(pub gcode<Span, GCode>,
    alt_complete!(
        // TODO: Handle `G00`
        map!(tag_no_case!("G0"), |_| GCode::Rapid) |
        // TODO: Handle `G01`
        map!(tag_no_case!("G1"), |_| GCode::Feed) |
        map!(tag_no_case!("G21"), |_| GCode::UnitsMM) |
        map!(tag_no_case!("G20"), |_| GCode::UnitsInch) |
        map!(work_offset, GCode::WorkOffset) |
        map!(plane_select, GCode::PlaneSelect) |
        map!(dwell, GCode::Dwell)
    )
);
