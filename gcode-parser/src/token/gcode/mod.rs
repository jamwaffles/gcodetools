mod work_offset;

use self::work_offset::work_offset;
pub use self::work_offset::{WorkOffset, WorkOffsetValue};
use crate::Span;
use nom::*;

/// A G-code
#[derive(Debug, PartialEq, Clone)]
pub enum GCode<'a> {
    /// Rapid move
    Rapid(Rapid<'a>),

    /// Move at a defined feedrate
    Feed(Feed<'a>),

    /// Work offset (`G54`, `G55`, etc)
    WorkOffset(WorkOffset<'a>),
}

/// Rapid move
#[derive(Debug, PartialEq, Clone)]
pub struct Rapid<'a> {
    /// Position in source input
    pub span: Span<'a>,
}

/// Move at a defined feedrate
#[derive(Debug, PartialEq, Clone)]
pub struct Feed<'a> {
    /// Position in source input
    pub span: Span<'a>,
}

named!(pub gcode<Span, GCode>,
    alt_complete!(
        // TODO: Handle `G00`
        positioned!(tag_no_case!("G0"), |(span, _)| GCode::Rapid(Rapid { span })) |
        // TODO: Handle `G01`
        positioned!(tag_no_case!("G1"), |(span, _)| GCode::Feed(Feed { span })) |
        map!(work_offset, GCode::WorkOffset)
    )
);
