mod work_offset;

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
}

named!(pub gcode<Span, GCode>,
    alt_complete!(
        // TODO: Handle `G00`
        map!(tag_no_case!("G0"), |_| GCode::Rapid) |
        // TODO: Handle `G01`
        map!(tag_no_case!("G1"), |_| GCode::Feed) |
        map!(work_offset, GCode::WorkOffset)
    )
);
