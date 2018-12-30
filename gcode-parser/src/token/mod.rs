//! Tokens

pub(crate) mod arc;
pub(crate) mod comment;
pub(crate) mod coord;
pub(crate) mod gcode;
pub(crate) mod mcode;
pub(crate) mod othercode;

use self::arc::center_format_arc;
pub use self::arc::CenterFormatArc;
use self::comment::comment;
pub use self::comment::Comment;
use self::coord::coord;
pub use self::coord::Coord;
use self::gcode::gcode;
pub use self::gcode::{GCode, WorkOffset, WorkOffsetValue};
use self::mcode::mcode;
pub use self::mcode::MCode;
use self::othercode::{feedrate, line_number, spindle_speed, tool_number};
pub use self::othercode::{Feedrate, LineNumber, SpindleSpeed, ToolNumber};
use crate::parsers::code_number;
use crate::Span;
use nom::*;
use nom_locate::position;

/// Any possible token type recgonised by this parser
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    /// Any G-code
    GCode(GCode),

    /// Any M-code
    MCode(MCode),

    /// A coordinate consisting of at least one XYZUVWABC component
    Coord(Coord),

    /// The coordinates and offsets that define a clockwise (G2) or counterclockwise (G3) center
    /// format arc
    CenterFormatArc(CenterFormatArc),

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
}

/// An unknown token
#[derive(Debug, PartialEq, Clone)]
pub struct Unknown {
    /// Code letter (`'G'`, `'M'`, etc)
    pub code_letter: char,

    /// Code number
    ///
    /// For G54, this would be `54.0`. For `G33.1`, this would be `33.1`
    pub code_number: f32,
}

/// Parsed GCode token
#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    /// Position in the source file at which this token occurs
    pub span: Span<'a>,

    /// The type and value of this token
    pub token: TokenType,
}

named!(unknown<Span, Unknown>,
    map!(
        tuple!(one_of!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"), code_number),
        |(code_letter, code_number)| Unknown { code_letter, code_number }
    )
);

named!(token_type<Span, TokenType>,
    alt_complete!(
        map!(center_format_arc, TokenType::CenterFormatArc) |
        map!(coord, TokenType::Coord) |
        map!(gcode, TokenType::GCode) |
        map!(mcode, TokenType::MCode) |
        map!(feedrate, TokenType::Feedrate) |
        map!(spindle_speed, TokenType::SpindleSpeed) |
        map!(tool_number, TokenType::ToolNumber) |
        map!(comment, TokenType::Comment) |
        map!(line_number, TokenType::LineNumber) |
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
