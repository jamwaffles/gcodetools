mod coord;
mod gcode;
mod mcode;
mod othercode;

use self::coord::coord;
pub use self::coord::Coord;
use self::gcode::gcode;
pub use self::gcode::GCode;
use self::mcode::mcode;
pub use self::mcode::MCode;
use self::othercode::othercode;
pub use self::othercode::OtherCode;
use crate::Span;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq)]
pub enum TokenType<'a> {
    GCode(GCode<'a>),
    MCode(MCode<'a>),
    Coord(Coord<'a>),
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
		map!(othercode, TokenType::OtherCode)
	)
);

named!(pub token<Span, Token>,
	do_parse!(
		span: position!() >>
		token: token_type >>
		(Token { span, token })
	)
);
