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
        map!(call!(g, 93.0), |_| FeedrateMode::InverseTime) |
        map!(call!(g, 94.0), |_| FeedrateMode::UnitsPerMinute) |
        map!(call!(g, 95.0), |_| FeedrateMode::UnitsPerRevolution)
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
