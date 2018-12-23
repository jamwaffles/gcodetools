mod program;

pub use self::program::{program, Program};
use crate::Span;
use nom::*;

#[derive(Debug, PartialEq)]
pub enum Block<'a> {
    Program(Program<'a>),
}

named!(pub block<Span, Block>,
	alt_complete!(
		map!(program, Block::Program)
	)
);
