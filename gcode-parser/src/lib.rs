// TODO: Enforce documentation for both pub and private things

use nom::types::CompleteByteSlice;
use nom_locate::LocatedSpan;

#[macro_use]
mod macros;
mod line;
mod parsers;
mod token;

type Span<'a> = LocatedSpan<CompleteByteSlice<'a>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
