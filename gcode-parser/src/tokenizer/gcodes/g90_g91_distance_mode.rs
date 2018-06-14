use nom::types::CompleteByteSlice;

use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum DistanceMode {
    Absolute,
    Incremental,
}

named!(pub distance_mode<CompleteByteSlice, Token>, map!(
    alt!(
        g_int!(90, DistanceMode::Absolute) |
        g_int!(91, DistanceMode::Incremental)
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
