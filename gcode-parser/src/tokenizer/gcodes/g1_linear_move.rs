use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

named!(pub linear_move<CompleteByteSlice, Token>,
    map!(call!(g, 1.0), |_| Token::LinearMove)
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
    fn it_parses_linear_moves() {
        check_token(linear_move(Cbs(b"G1")), Token::LinearMove);
        check_token(linear_move(Cbs(b"G01")), Token::LinearMove);
    }
}
