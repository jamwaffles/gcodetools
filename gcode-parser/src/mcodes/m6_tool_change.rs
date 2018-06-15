use nom::types::CompleteByteSlice;

use super::MCode;

named!(pub tool_change<CompleteByteSlice, MCode>,
    m_code!("6", MCode::ToolChange)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_token(
        to_check: Result<(CompleteByteSlice, MCode), nom::Err<CompleteByteSlice>>,
        against: MCode,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_tool_changes() {
        check_token(tool_change(Cbs(b"M6")), MCode::ToolChange);
    }
}
