use nom::types::CompleteByteSlice;

use super::super::Token;

/// Units selection
#[derive(Debug, PartialEq)]
pub enum Units {
    /// Use inch units for all distances
    Inch,
    /// Use millimeter units for all distances
    Mm,
}

named!(pub units<CompleteByteSlice, Token>, map!(
    alt!(
        g_int!(20, Units::Inch) |
        g_int!(21, Units::Mm)
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
