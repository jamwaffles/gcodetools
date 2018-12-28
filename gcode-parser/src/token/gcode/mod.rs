mod arc;
mod dwell;
mod plane_select;
mod work_offset;

use self::arc::arc;
pub use self::arc::Arc;
use self::dwell::dwell;
pub use self::dwell::Dwell;
use self::plane_select::plane_select;
pub use self::plane_select::PlaneSelect;
use self::work_offset::work_offset;
pub use self::work_offset::{WorkOffset, WorkOffsetValue};
use crate::{map_code, Span};
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

    /// A clockwise or counterclockwise arc
    Arc(Arc),
}

named!(pub gcode<Span, GCode>,
    alt_complete!(
        map_code!("G0", |_| GCode::Rapid) |
        map_code!("G1", |_| GCode::Feed) |
        map!(arc, GCode::Arc) |
        map_code!("G21", |_| GCode::UnitsMM) |
        map_code!("G20", |_| GCode::UnitsInch) |
        map!(work_offset, GCode::WorkOffset) |
        map!(plane_select, GCode::PlaneSelect) |
        map!(dwell, GCode::Dwell)
    )
);
