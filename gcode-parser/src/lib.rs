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

use crate::block::Block;
use nom::types::CompleteByteSlice;
use nom::*;
use nom_locate::LocatedSpan;

#[macro_use]
mod macros;
mod block;
mod line;
mod parsers;
mod token;

use crate::block::block;

named!(parse_program_tree<Span, Block>,
    map_res!(
        block,
        |result| {
            match result {
                Block::Program(p) => Ok(Block::Program(p)),
                _ => Err(())
            }
        }
    )
);

/// Take an input span and parse it into a tree of tokens
pub fn from_str(input: &str) -> Result<Block, String> {
    let input = Span::new(CompleteByteSlice(input.as_bytes()));

    // TODO: Better handling of `remaining` errors
    parse_program_tree(input)
        .map_err(|e| format!("Failed to parse program: {}", e))
        .map(|(_remaining, program)| program)
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
