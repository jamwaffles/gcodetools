use nom::types::CompleteByteSlice;

use super::super::Token;

named!(pub tool_change<CompleteByteSlice, Token>,
    m_int!(6, Token::ToolChange)
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
    fn it_parses_tool_changes() {
        check_token(tool_change(Cbs(b"M6")), Token::ToolChange);
    }
}
