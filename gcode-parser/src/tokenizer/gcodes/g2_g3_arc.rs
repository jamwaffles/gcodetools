use nom::types::CompleteByteSlice;

use super::super::Token;

named!(pub arc<CompleteByteSlice, Token>, alt!(
    g_int!(2, Token::ClockwiseArc) |
    g_int!(3, Token::CounterclockwiseArc)
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
