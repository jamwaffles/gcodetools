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

impl<'a> Tokenizer<'a> {
    pub fn new_from_str(program_string: &'a str) -> Self {
        Tokenizer { program_string }
    }

    pub fn tokenize(&self) -> Result<(CompleteByteSlice, Program), nom::Err<CompleteByteSlice>> {
        program(CompleteByteSlice(self.program_string.as_bytes()))
    }
}

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
}

pub type Program = Vec<Token>;

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

named!(pub program<CompleteByteSlice, Program>, ws!(
    preceded!(
        opt!(tag!("%")),
        tokens
    )
));
