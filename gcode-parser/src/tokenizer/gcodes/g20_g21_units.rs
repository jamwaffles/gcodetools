use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum Units {
    Inch,
    Mm,
}

named!(pub units<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 20.0), |_| Units::Inch) |
        map!(call!(g, 21.0), |_| Units::Mm)
    ),
    |res| Token::Units(res)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_token(
        to_check: Result<(CompleteByteSlice, Token), nom::Err<CompleteByteSlice>>,
        against: Token,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_units() {
        check_token(units(Cbs(b"G20")), Token::Units(Units::Inch));
        check_token(units(Cbs(b"G21")), Token::Units(Units::Mm));
    }
}
