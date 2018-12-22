mod coord;
mod gcode;
mod mcode;

use self::coord::coord;
pub use self::coord::Coord;
use self::gcode::gcode;
pub use self::gcode::GCode;
use self::mcode::mcode;
pub use self::mcode::MCode;
use crate::Span;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq)]
pub enum TokenType<'a> {
    GCode(GCode<'a>),
    MCode(MCode<'a>),
    Coord(Coord<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) token: TokenType<'a>,
}

named!(token_type<Span, TokenType>,
	alt_complete!(
		map!(gcode, TokenType::GCode) |
		map!(mcode, TokenType::MCode) |
		map!(coord, TokenType::Coord)
	)
);

named!(pub token<Span, Token>,
	do_parse!(
		span: position!() >>
		token: token_type >>
		(Token { span, token })
	)
);
