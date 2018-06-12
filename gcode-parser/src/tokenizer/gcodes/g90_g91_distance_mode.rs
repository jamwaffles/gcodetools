use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum DistanceMode {
    Absolute,
    Incremental,
}

named!(pub distance_mode<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 90.0), |_| DistanceMode::Absolute) |
        map!(call!(g, 91.0), |_| DistanceMode::Incremental)
    ),
    |res| Token::DistanceMode(res)
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
    fn it_parses_distance_mode() {
        check_token(
            distance_mode(Cbs(b"G90")),
            Token::DistanceMode(DistanceMode::Absolute),
        );

        check_token(
            distance_mode(Cbs(b"G91")),
            Token::DistanceMode(DistanceMode::Incremental),
        );
    }
}
