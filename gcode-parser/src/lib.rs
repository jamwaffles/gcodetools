//! A GCode parser written using Nom macros
//!
//! This parser aims to be able to parse all valid GCode programs, but bases its grammar on the
//! LinuxCNC [G-code](http://linuxcnc.org/docs/html/gcode/g-code.html),
//! [M-code](http://linuxcnc.org/docs/html/gcode/m-code.html),
//! [O-code](http://linuxcnc.org/docs/html/gcode/o-code.html) and
//! [other code](http://linuxcnc.org/docs/html/gcode/other-code.html) definitions.

#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use crate::block::{program, Block, Program as ProgramBlock};
use nom::types::CompleteByteSlice;
use nom::*;
use nom_locate::LocatedSpan;
use std::{fs, io};

#[macro_use]
mod macros;
mod block;
mod line;
mod parsers;
mod token;

use crate::block::block;

named!(parse_program_tree<Span, ProgramBlock>,
    map_res!(
        block,
        |result| {
            match result {
                Block::Program(p) => Ok(p),
                _ => Err(())
            }
        }
    )
);

/// Container for a complete GCode program, including sub-programs
#[derive(Debug)]
pub struct Program<'a> {
    token_tree: ProgramBlock<'a>,
}

impl<'a> Program<'a> {
    /// Parse a GCode program from a given string
    pub fn from_str(content: &'a str) -> Result<Self, io::Error> {
        let (_remaining, token_tree) =
            parse_program_tree(Span::new(CompleteByteSlice(content.as_bytes())))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(Self { token_tree })
    }
}

#[doc(hidden)]
pub type Span<'a> = LocatedSpan<CompleteByteSlice<'a>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
