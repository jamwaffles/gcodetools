use nom::types::CompleteByteSlice;

use super::MCode;

named!(pub tool_change<CompleteByteSlice, MCode>,
    m_code!("6", MCode::ToolChange)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_tool_changes() {
        assert_complete_parse!(tool_change(Cbs(b"M6")), MCode::ToolChange);
    }
}
