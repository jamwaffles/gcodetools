mod coord;
mod gcode;
mod mcode;
mod othercode;
mod program;

use self::coord::coord;
pub use self::coord::Coord;
use self::gcode::gcode;
pub use self::gcode::GCode;
use self::mcode::mcode;
pub use self::mcode::MCode;
use self::othercode::othercode;
pub use self::othercode::OtherCode;
use self::program::program;
pub use self::program::Program;
use crate::Span;
use nom::*;
use nom_locate::position;

/// Any possible token type recgonised by this parser
#[derive(Debug, PartialEq)]
pub enum TokenType<'a> {
    /// Any G-code
    GCode(GCode<'a>),

    /// Any M-code
    MCode(MCode<'a>),

    /// A coordinate consisting of at least one XYZUVWABC component
    Coord(Coord<'a>),

    /// A complete program listing
    ///
    /// This may be at the top level, or hold a sub-program referred to by an O-code
    Program(Program<'a>),

    /// An F-, S- or T-code
    OtherCode(OtherCode<'a>),
}

/// Parsed GCode token
#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) token: TokenType<'a>,
}

named!(token_type<Span, TokenType>,
    alt_complete!(
        map!(gcode, TokenType::GCode) |
        map!(coord, TokenType::Coord) |
        map!(mcode, TokenType::MCode) |
        map!(othercode, TokenType::OtherCode) |
        map!(program, TokenType::Program)
    )
);

named!(pub token<Span, Token>,
    do_parse!(
        span: position!() >>
        token: token_type >>
        (Token { span, token })
    )
);
