use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

named!(pub end_program<CompleteByteSlice, Token>, map!(
    alt!(
        recognize!(call!(m, 30.0)) |
        recognize!(call!(m, 2.0))
    ),
    |_| Token::EndProgram
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
    fn it_parses_end_program() {
        check_token(end_program(Cbs(b"M2")), Token::EndProgram);
        check_token(end_program(Cbs(b"M30")), Token::EndProgram);
    }
}
