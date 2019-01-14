//! Tokens

pub(crate) mod arc;
pub(crate) mod assignment;
pub(crate) mod block;
pub(crate) mod call;
pub(crate) mod comment;
pub(crate) mod coord;
pub(crate) mod gcode;
pub(crate) mod mcode;
pub(crate) mod othercode;
pub(crate) mod polar;
pub(crate) mod return_stmt;

use self::arc::{center_format_arc, radius_format_arc};
pub use self::arc::{CenterFormatArc, RadiusFormatArc};
use self::assignment::assignment;
pub use self::assignment::Assignment;
use self::block::block;
pub use self::block::{Block, Branch, BranchType, Conditional, Repeat, Subroutine, While};
use self::call::call;
pub use self::call::Call;
use self::comment::comment;
pub use self::comment::Comment;
use self::coord::coord;
pub use self::coord::Coord;
use self::gcode::gcode;
pub use self::gcode::{CutterCompensation, GCode, WorkOffset, WorkOffsetValue};
use self::mcode::mcode;
pub use self::mcode::MCode;
use self::othercode::{feedrate, line_number, spindle_speed, tool_number};
pub use self::othercode::{Feedrate, LineNumber, SpindleSpeed, ToolNumber};
use self::polar::polar;
pub use self::polar::PolarCoord;
use self::return_stmt::return_stmt;
pub use self::return_stmt::Return;
use common::parsing::Span;
use expression::{parser::ngc_float_value, Value};
use nom::*;
use nom_locate::position;

/// Any possible token type recgonised by this parser
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType<'a> {
    /// Any G-code
    GCode(GCode),

    /// Any M-code
    MCode(MCode),

    /// A coordinate consisting of at least one XYZUVWABC component
    Coord(Coord),

    /// A polar coordinate defined by distance and angle from current position
    PolarCoord(PolarCoord),

    /// The coordinates and offsets that define a clockwise (G2) or counterclockwise (G3) center
    /// format arc
    CenterFormatArc(CenterFormatArc),

    /// Radius-format arc
    RadiusFormatArc(RadiusFormatArc),

    /// Feedrate
    Feedrate(Feedrate),

    /// Spindle speed
    SpindleSpeed(SpindleSpeed),

    /// Tool number
    ToolNumber(ToolNumber),

    /// A comment
    Comment(Comment),

    /// A line number
    LineNumber(LineNumber),

    /// A code that this parser doesn't understand
    Unknown(Unknown),

    /// An assignment of a literal, parameter or expression to a parameter
    Assignment(Assignment),

    /// A block (subrouting, while loop, if statement, etc)
    Block(Block<'a>),

    /// A subroutine call
    Call(Call),

    /// A return statement
    Return(Return),
}

/// An unknown token
#[derive(Debug, PartialEq, Clone)]
pub struct Unknown {
    /// Code letter (`'G'`, `'M'`, etc)
    pub code_letter: char,

    /// Code number
    pub code_number: Value,
}

/// Parsed GCode token
#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    /// Position in the source file at which this token occurs
    pub span: Span<'a>,

    /// The type and value of this token
    pub token: TokenType<'a>,
}

named!(unknown<Span, Unknown>,
    map!(
        sep!(
            space0,
            tuple!(one_of!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"), ngc_float_value)
        ),
        |(code_letter, code_number)| Unknown { code_letter, code_number }
    )
);

named!(token_type<Span, TokenType>,
    alt!(
        map!(center_format_arc, TokenType::CenterFormatArc) |
        map!(radius_format_arc, TokenType::RadiusFormatArc) |
        map!(coord, TokenType::Coord) |
        map!(gcode, TokenType::GCode) |
        map!(mcode, TokenType::MCode) |
        map!(feedrate, TokenType::Feedrate) |
        map!(spindle_speed, TokenType::SpindleSpeed) |
        map!(tool_number, TokenType::ToolNumber) |
        map!(comment, TokenType::Comment) |
        map!(line_number, TokenType::LineNumber) |
        map!(assignment, TokenType::Assignment) |
        map!(block, TokenType::Block) |
        map!(call, TokenType::Call) |
        map!(return_stmt, TokenType::Return) |
        map!(polar, TokenType::PolarCoord) |
        map!(unknown, TokenType::Unknown)
    )
);

named!(pub(crate) token<Span, Token>,
    do_parse!(
        span: position!() >>
        token: token_type >>
        (Token { span, token })
    )
);
