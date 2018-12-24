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

#[macro_use]
mod macros;
mod line;
mod parsers;
mod token;

use crate::token::token;
pub use crate::token::{Token, TokenType};
use nom::types::CompleteByteSlice;
use nom::InputLength;
use nom_locate::LocatedSpan;
use std::io;

/// Container for a complete GCode program, including sub-programs
#[derive(Debug)]
pub struct Program<'a> {
    token_tree: Token<'a>,
}

impl<'a> Program<'a> {
    /// Parse a GCode program from a given string
    pub fn from_str(content: &'a str) -> Result<Self, io::Error> {
        let (remaining, token_tree) = token(Span::new(CompleteByteSlice(content.as_bytes())))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // TODO: Return a better error type
        if remaining.input_len() > 0 {
            let line = remaining.line;
            let column = remaining.get_column();

            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Could not parse complete program, failed at line {} col {} (byte {} of {})",
                    line,
                    column,
                    remaining.input_len(),
                    content.len()
                ),
            ))
        } else {
            Ok(Self { token_tree })
        }
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
