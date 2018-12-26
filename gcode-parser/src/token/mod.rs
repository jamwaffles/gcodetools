mod coord;
mod gcode;
mod mcode;
mod othercode;

use self::coord::coord;
pub use self::coord::Coord;
use self::gcode::gcode;
pub use self::gcode::{GCode, RawGCode};
use self::mcode::mcode;
pub use self::mcode::MCode;
use self::othercode::{feedrate, spindle_speed, tool_number};
pub use self::othercode::{Feedrate, SpindleSpeed, ToolNumber};
use crate::Span;
use nom::*;
use nom_locate::position;

/// Any possible token type recgonised by this parser
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType<'a> {
    /// Any G-code
    GCode(GCode<'a>),

    /// Any M-code
    MCode(MCode<'a>),

    /// A coordinate consisting of at least one XYZUVWABC component
    Coord(Coord<'a>),

    /// Feedrate
    Feedrate(Feedrate<'a>),

    /// Spindle speed
    SpindleSpeed(SpindleSpeed<'a>),

    /// Tool number
    ToolNumber(ToolNumber<'a>),
}

/// Parsed GCode token
#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    /// Position in the source file at which this token occurs
    pub span: Span<'a>,

    /// The type and value of this token
    pub token: TokenType<'a>,
}

named!(token_type<Span, TokenType>,
    alt_complete!(
        map!(gcode, TokenType::GCode) |
        map!(coord, TokenType::Coord) |
        map!(mcode, TokenType::MCode) |
        map!(feedrate, TokenType::Feedrate) |
        map!(spindle_speed, TokenType::SpindleSpeed) |
        map!(tool_number, TokenType::ToolNumber)
    )
);

named!(pub token<Span, Token>,
    do_parse!(
        span: position!() >>
        token: token_type >>
        (Token { span, token })
    )
);
