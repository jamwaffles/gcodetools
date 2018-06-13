use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum FeedrateMode {
    InverseTime,
    UnitsPerMinute,
    UnitsPerRevolution,
}

named!(pub feedrate_mode<CompleteByteSlice, Token>, map!(
    alt!(
        g_int!(93, FeedrateMode::InverseTime) |
        g_int!(94, FeedrateMode::UnitsPerMinute) |
        g_int!(95, FeedrateMode::UnitsPerRevolution)
    ),
    |res| Token::FeedrateMode(res)
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
    fn it_parses_feedrate_mode() {
        check_token(
            feedrate_mode(Cbs(b"G93")),
            Token::FeedrateMode(FeedrateMode::InverseTime),
        );
        check_token(
            feedrate_mode(Cbs(b"G94")),
            Token::FeedrateMode(FeedrateMode::UnitsPerMinute),
        );
        check_token(
            feedrate_mode(Cbs(b"G95")),
            Token::FeedrateMode(FeedrateMode::UnitsPerRevolution),
        );
    }
}
