use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

named!(pub canned_cycle<CompleteByteSlice, Token>,
    alt!(
        map!(call!(g, 80.0), |_| Token::CancelCannedCycle)
    )
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
    fn it_parses_canned_cycles() {
        check_token(canned_cycle(Cbs(b"G80")), Token::CancelCannedCycle);
    }
}
