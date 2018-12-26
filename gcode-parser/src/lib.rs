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
mod program;
pub mod token;

pub use crate::program::Program;
use nom::types::CompleteByteSlice;
use nom_locate::LocatedSpan;

#[doc(hidden)]
pub type Span<'a> = LocatedSpan<CompleteByteSlice<'a>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
