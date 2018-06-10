mod arc;
mod comment;
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
use super::expression::Expression;
use super::subroutine::{
    parser::{control_flow, subroutine}, If, Subroutine, SubroutineCall, While,
};

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
    BlockDelete(Vec<Token>),
    CancelCannedCycle,
    CenterArc(CenterArc),
    ClockwiseArc,
    Comment(String),
    Coolant(Coolant),
    Coord(Vec9),
    CoordinateSystemOffset,
    CoordinateSystemOffsetHardReset,
    CoordinateSystemOffsetSoftReset,
    CounterclockwiseArc,
    CutterCompensation(CutterCompensation),
    DistanceMode(DistanceMode),
    Dwell(f32),
    EndProgram,
    FeedRate(Value),
    FeedrateMode(FeedrateMode),
    GlobalMove,
    GoToPredefinedPosition(PredefinedPosition),
    If(If),
    LatheMeasurementMode(LatheMeasurementMode),
    LinearMove,
    LineNumber(u32),
    ModalStateAutoRestore,
    ModalStateInvalidate,
    ModalStateRestore,
    ModalStateSave,
    OptionalPause,
    Parameter(Parameter),
    ParameterAssignment((Parameter, Expression)),
    PathBlendingMode(PathBlendingMode),
    Pause,
    PlaneSelect(Plane),
    RadiusArc(RadiusArc),
    RapidMove,
    SpindleRotation(SpindleRotation),
    SpindleSpeed(Value),
    StorePredefinedPosition(PredefinedPosition),
    StraightProbe(StraightProbe),
    SubroutineCall(SubroutineCall),
    SubroutineDefinition(Subroutine),
    ToolChange,
    ToolLengthCompensation(ToolLengthCompensation),
    ToolLengthCompensationToolNumber(Value),
    ToolSelect(Value),
    Units(Units),
    UserCommand(u32),
    While(While),
    WorkOffset(WorkOffset),
}

/// List of parsed GCode tokens
pub type ProgramTokens = Vec<Token>;

// Subroutines can't be nested
named!(pub token_not_subroutine<CompleteByteSlice, Token>,
    alt_complete!(
        block_delete |
        gcode |
        mcode |
        othercode |
        arc |
        coord |
        comment |
        parameters |
        control_flow
    )
);

named!(token<CompleteByteSlice, Token>,
    alt_complete!(
        token_not_subroutine |
        subroutine
    )
);

named!(block_delete<CompleteByteSlice, Token>, map!(
    flat_map!(
        delimited!(tag!("/"), take_until_line_ending, line_ending),
        tokens
    ),
    |tokens| Token::BlockDelete(tokens)
));

named!(tokens<CompleteByteSlice, Vec<Token>>, ws!(many0!(token)));

/// Raw GCode parser
///
/// This should only be used for testing purposes. The proper interface to the tokenizer is the
/// [Tokenizer] struct.
named!(pub program<CompleteByteSlice, ProgramTokens>, ws!(
    delimited!(
        opt!(tag!("%")),
        tokens,
        opt!(tag!("%"))
    )
));

pub mod test_prelude;
