//! A single GCode literal like `G38.1` or `G0`

use crate::parsers::char_no_case;
use nom::character::complete::digit1;
use nom::number::complete::recognize_float;
use nom::{
    combinator::{recognize, verify},
    error::ParseError,
    sequence::pair,
    IResult,
};

/// Parse a word denoted by an integer number, e.g. `G0`.
pub fn word<'a, E: ParseError<&'a str>>(
    search: &'a str,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, E> {
    let (letter, rest) = search.split_at(1);
    let letter = letter.as_bytes()[0] as char;

    let number = rest.parse::<u16>().unwrap();

    recognize(verify(
        pair(char_no_case(letter), digit1),
        move |(_, n): &(_, &str)| n.parse::<u16>().unwrap() == number,
    ))
}

/// Parse a word denoted by a decimal number, e.g. `G43.1`.
///
/// Leading zeros are not supported.
pub fn decimal_word<'a, E: ParseError<&'a str>>(
    search: &'a str,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, E> {
    let (letter, rest) = search.split_at(1);
    let letter = letter.as_bytes()[0] as char;

    recognize(verify(
        pair(char_no_case(letter), recognize_float),
        move |(_, number): &(_, &str)| number == &rest,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::VerboseError;

    #[test]
    fn integer_word() {
        let (remaining, p) = word::<VerboseError<&str>>("G34")("G34").unwrap();

        assert_eq!(remaining, "");
        assert_eq!(p, "G34");
    }

    #[test]
    fn parse_decimal_word() {
        let (remaining, p) = decimal_word::<VerboseError<&str>>("G38.5")("g38.5").unwrap();

        assert_eq!(remaining, "");
        assert_eq!(p, "g38.5");
    }

    #[test]
    fn leading_zeros() {
        let (remaining, p) = word::<VerboseError<&str>>("G1")("G01").unwrap();

        assert_eq!(remaining, "");
        assert_eq!(p, "G01");
    }

    #[test]
    fn partial_match() {
        assert!(word::<VerboseError<&str>>("M61")("M6T").is_err());
    }

    #[test]
    fn trailing_crap() {
        let (remaining, p) = decimal_word::<VerboseError<&str>>("G38.5")("g38.5   g1").unwrap();

        assert_eq!(remaining, "   g1");
        assert_eq!(p, "g38.5");
    }
}
