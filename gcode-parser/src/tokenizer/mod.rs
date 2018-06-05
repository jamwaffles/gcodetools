mod arc;
mod comment;
// FIXME: Should not be pub, move to ../expression
pub mod expression;
mod gcodes;
// FIXME: Should not be pub
pub mod helpers;
mod mcodes;
mod othercodes;
// FIXME: Should not be pub
pub mod parameter;
pub mod prelude;
mod value;
mod vec9;

use nom;
use nom::types::CompleteByteSlice;
use nom::*;

use self::arc::*;
use self::comment::*;
use self::gcodes::*;
use self::helpers::*;
use self::mcodes::*;
use self::othercodes::*;
use self::parameter::*;
use self::value::*;
use self::vec9::*;

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
    CenterArc(CenterArc),
    RadiusArc(RadiusArc),
    Coord(Vec9),
    ToolSelect(Value),
    ToolChange,
    PlaneSelect(Plane),
    SpindleRotation(SpindleRotation),
    SpindleSpeed(Value),
    FeedRate(Value),
    LineNumber(u32),
    Coolant(Coolant),
    ToolLengthCompensation(ToolLengthCompensation),
    ClockwiseArc,
    CounterclockwiseArc,
    ToolLengthCompensationToolNumber(Value),
    CancelCannedCycle,
    EndProgram,
    WorkOffset(WorkOffset),
    Dwell(f32),
    CoordinateSystemOffset,
    FeedrateMode(FeedrateMode),
    GoToPredefinedPosition,
    StorePredefinedPosition,
    Pause,
    BlockDelete(Vec<Token>),
    // TODO: ParameterValue needs to become an Expression for general purpose float/expression support
    ParameterAssignment((Parameter, ParameterValue)),
    Parameter(Parameter),
    ModalStateSave,
    ModalStateRestore,
    ModalStateInvalidate,
    ModalStateAutoRestore,
}

/// List of parsed GCode tokens
pub type ProgramTokens = Vec<Token>;

named!(token<CompleteByteSlice, Token>,
    alt_complete!(
        block_delete |
        gcode |
        mcode |
        othercode |
        arc |
        coord |
        comment |
        end_program |
        parameters
    )
);

named!(tokens<CompleteByteSlice, Vec<Token>>, ws!(many0!(token)));

named!(block_delete<CompleteByteSlice, Token>, map!(
    flat_map!(
        delimited!(tag!("/"), take_until_line_ending, line_ending),
        tokens
    ),
    |tokens| Token::BlockDelete(tokens)
));

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
