mod program;

use self::program::{program, Program};
use nom::types::CompleteByteSlice;
use nom::*;

#[derive(Debug, PartialEq)]
pub enum Block {
    Program(Program),
}

named!(pub block<CompleteByteSlice, Block>,
	alt_complete!(
		map!(program, |res| Block::Program(res))
	)
);
