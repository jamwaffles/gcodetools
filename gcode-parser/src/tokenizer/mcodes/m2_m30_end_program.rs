use nom::types::CompleteByteSlice;

use super::super::Token;

named!(pub end_program<CompleteByteSlice, Token>, alt!(
    m_int!(30, Token::EndProgram) |
    m_int!(2, Token::EndProgram)
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
