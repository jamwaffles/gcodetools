use nom::types::CompleteByteSlice;

use super::super::Token;

named!(pub pause<CompleteByteSlice, Token>, alt!(
    m_int!(0, Token::Pause) |
    m_int!(1, Token::OptionalPause)
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
    fn it_parses_pauses() {
        check_token(pause(Cbs(b"M0")), Token::Pause);
        check_token(pause(Cbs(b"M00")), Token::Pause);

        check_token(pause(Cbs(b"M1")), Token::OptionalPause);
        check_token(pause(Cbs(b"M01")), Token::OptionalPause);
    }
}
