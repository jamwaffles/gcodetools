use nom::types::CompleteByteSlice;

use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum WorkOffset {
    G54,
    G55,
}

named!(pub work_offset<CompleteByteSlice, Token>, map!(
    alt!(
        g_int!(54, WorkOffset::G54) |
        g_int!(55, WorkOffset::G55)
    ),
    |res| Token::WorkOffset(res)
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
    fn it_parses_work_offsets() {
        check_token(work_offset(Cbs(b"G54")), Token::WorkOffset(WorkOffset::G54));
        check_token(work_offset(Cbs(b"G55")), Token::WorkOffset(WorkOffset::G55));
    }
}
