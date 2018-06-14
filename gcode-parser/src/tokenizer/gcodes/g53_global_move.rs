use nom::types::CompleteByteSlice;

use super::super::Token;

named!(pub global_move<CompleteByteSlice, Token>,
    g_int!(53, Token::GlobalMove)
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
    fn it_parses_global_moves() {
        check_token(global_move(Cbs(b"G53")), Token::GlobalMove);
    }
}
