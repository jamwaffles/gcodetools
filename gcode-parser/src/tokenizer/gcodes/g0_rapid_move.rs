use nom::types::CompleteByteSlice;

use super::super::Token;

named!(pub rapid_move<CompleteByteSlice, Token>,
    g_int!(0, Token::RapidMove)
);

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
    fn it_parses_rapids() {
        check_token(rapid_move(Cbs(b"G0")), Token::RapidMove);
        check_token(rapid_move(Cbs(b"G00")), Token::RapidMove);
    }
}
