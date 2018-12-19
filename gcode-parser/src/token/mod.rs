mod block;
mod coord;
mod gcode;
mod mcode;

use self::coord::coord;
pub use self::coord::Coord;
use self::gcode::gcode;
pub use self::gcode::GCode;
use self::mcode::mcode;
pub use self::mcode::MCode;
use nom::types::CompleteByteSlice;
use nom::*;

// TODO: Delete
// #[derive(Debug, PartialEq)]
// pub enum CodeNumber {
//     Int(u16),
//     Float(f32),
// }

#[derive(Debug, PartialEq)]
pub enum Token {
    GCode(GCode),
    MCode(MCode),
    Coord(Coord),
}

named!(pub token<CompleteByteSlice, Token>,
	alt_complete!(
		map!(gcode, Token::GCode) |
		map!(mcode, Token::MCode) |
		map!(coord, Token::Coord)
	)
);
