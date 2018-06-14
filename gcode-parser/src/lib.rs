//! Gcode parser

#![deny(/*missing_docs,*/
        missing_debug_implementations, /*missing_copy_implementations,*/
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces/*, unused_qualifications*/)]

#[macro_use]
#[cfg(test)]
extern crate maplit;

#[macro_use]
extern crate nom;

#[macro_use]
mod macros;

mod arc;
mod comment;
mod gcodes;
// FIXME: Should not be pub
pub mod helpers;
mod mcodes;
mod othercodes;
// FIXME: Should not be pub
mod expression;
pub mod parameter;
pub mod prelude;
mod subroutine;
mod value;
mod vec9;

use nom::types::CompleteByteSlice;
use nom::*;

use self::arc::*;
use self::comment::*;
use self::gcodes::*;
use self::helpers::*;
use self::mcodes::*;
use self::othercodes::*;
use self::parameter::*;
use self::subroutine::{
    parser::{control_flow, subroutine}, If, Repeat, Subroutine, SubroutineCall, While,
};
use self::value::*;
use self::vec9::*;

pub mod test_prelude;

/// Main interface to the parser
#[derive(Debug)]
pub struct Tokenizer<'a> {
    program_string: &'a str,
}

/// Main interface to the tokenizer
impl<'a> Tokenizer<'a> {
    /// Take an input str ready for parsing
    pub fn new_from_str(program_string: &'a str) -> Self {
        Tokenizer { program_string }
    }

    /// Parse a program into tokens
    pub fn tokenize(&self) -> Result<(CompleteByteSlice, ProgramTokens), Err<CompleteByteSlice>> {
        program(CompleteByteSlice(self.program_string.as_bytes()))
    }
}

/// Enum describing all supported GCode tokens
#[derive(Debug, PartialEq)]
pub enum Token {
    BlockDelete(Vec<Token>),
    CenterArc(CenterArc),
    Comment(String),
    Coord(Vec9),
    FeedRate(Value),
    If(If),
    LineNumber(u32),
    Parameter(Parameter),
    ParameterAssignment((Parameter, Value)),
    RadiusArc(RadiusArc),
    Repeat(Repeat),
    SpindleSpeed(Value),
    SubroutineCall(SubroutineCall),
    SubroutineDefinition(Subroutine),
    ToolSelect(Value),
    While(While),
    MCode(MCode),
    GCode(GCode),
    ToolLengthCompensationToolNumber(Value),
}

/// List of parsed GCode tokens
pub type ProgramTokens = Vec<Token>;

/// Match a line of optional code
named!(block_delete<CompleteByteSlice, Token>, map!(
    flat_map!(
        delimited!(tag!("/"), take_until_line_ending, line_ending),
        tokens
    ),
    |tokens| Token::BlockDelete(tokens)
));

/// Match any token that's not a subroutine
///
/// Subroutine definitions can't be nested (but calls can). Add any new parsers here.
named!(pub token_not_subroutine<CompleteByteSlice, Token>,
    alt_complete!(
        block_delete |
        map!(gcode, |m| Token::GCode(m)) |
        map!(mcode, |m| Token::MCode(m)) |
        othercode |
        arc |
        coord |
        comment |
        parameters |
        control_flow
    )
);

/// Match _any_ token
named!(token<CompleteByteSlice, Token>,
    alt_complete!(
        token_not_subroutine |
        subroutine
    )
);

/// Match a list of tokens that make up a program
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
