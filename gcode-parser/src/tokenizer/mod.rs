mod arc;
mod gcodes;
mod helpers;
mod mcodes;
mod othercodes;
pub mod prelude;

use nom;
use nom::types::CompleteByteSlice;

use self::arc::*;
use self::gcodes::*;
use self::helpers::*;
use self::mcodes::*;
use self::othercodes::*;

pub struct Tokenizer<'a> {
    program_string: &'a str,
}

/// Main interface to the tokenizer
impl<'a> Tokenizer<'a> {
    pub fn new_from_str(program_string: &'a str) -> Self {
        Tokenizer { program_string }
    }

    pub fn tokenize(
        &self,
    ) -> Result<(CompleteByteSlice, ProgramTokens), nom::Err<CompleteByteSlice>> {
        program(CompleteByteSlice(self.program_string.as_bytes()))
    }
}

/// Enum describing all supported GCode tokens
#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Units(Units),
    DistanceMode(DistanceMode),
    PathBlendingMode(PathBlendingMode),
    CutterCompensation(CutterCompensation),
    RapidMove,
    LinearMove,
    CenterFormatArc(CenterFormatArc),
    Coord(Vec9),
    ToolSelect(u32),
    ToolChange,
    PlaneSelect(Plane),
    SpindleRotation(SpindleRotation),
    SpindleSpeed(i32),
    FeedRate(f32),
    LineNumber(u32),
    Coolant(Coolant),
    ToolLengthCompensation(ToolLengthCompensation),
    ClockwiseArc,
    CounterclockwiseArc,
    ToolLengthCompensationToolNumber(u32),
    CancelCannedCycle,
    EndProgram,
    WorkOffset(WorkOffset),
    Dwell(f32),
    CoordinateSystemOffset,
    FeedrateMode(FeedrateMode),
    GoToPredefinedPosition,
    StorePredefinedPosition,
    Pause,
}

/// List of parsed GCode tokens
pub type ProgramTokens = Vec<Token>;

named!(token<CompleteByteSlice, Token>,
    alt_complete!(
        gcode |
        mcode |
        othercode |
        center_format_arc |
        coord |
        comment |
        end_program
    )
);

named!(tokens<CompleteByteSlice, Vec<Token>>, ws!(many0!(token)));

/// Raw GCode parser
///
/// This should only be used for testing purposes. The proper interface to the tokenizer is the
/// [Tokenizer] struct.
named!(pub program<CompleteByteSlice, ProgramTokens>, ws!(
    preceded!(
        opt!(tag!("%")),
        tokens
    )
));

pub mod test_prelude;
