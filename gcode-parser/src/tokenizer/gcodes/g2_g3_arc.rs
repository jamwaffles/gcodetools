use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

named!(pub arc<CompleteByteSlice, Token>, alt!(
    map!(call!(g, 2.0), |_| Token::ClockwiseArc) |
    map!(call!(g, 3.0), |_| Token::CounterclockwiseArc)
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
    fn it_parses_arcs() {
        check_token(arc(Cbs(b"G2")), Token::ClockwiseArc);
        check_token(arc(Cbs(b"G02")), Token::ClockwiseArc);
        check_token(arc(Cbs(b"G3")), Token::CounterclockwiseArc);
        check_token(arc(Cbs(b"G03")), Token::CounterclockwiseArc);
    }
}
