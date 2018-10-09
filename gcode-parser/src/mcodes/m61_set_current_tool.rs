use super::super::value::preceded_unsigned_value;
use nom::types::CompleteByteSlice;

use super::MCode;

named!(pub set_current_tool<CompleteByteSlice, MCode>, map!(
    ws!(tuple!(
        m_code!("61"),
        call!(preceded_unsigned_value, "Q")
    )),
    |(_, tool_number)| MCode::SetCurrentTool(tool_number)
));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;
    use expression::Parameter;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_set_current_tool() {
        assert_complete_parse!(
            set_current_tool(Cbs(b"M61Q#<n>")),
            MCode::SetCurrentTool(Value::Parameter(Parameter::Named("n".into())))
        );

        assert_complete_parse!(
            set_current_tool(Cbs(b"M61 Q10")),
            MCode::SetCurrentTool(Value::Unsigned(10))
        );

        assert_complete_parse!(
            set_current_tool(Cbs(b"M61 Q0")),
            MCode::SetCurrentTool(Value::Unsigned(0))
        );

        assert!(set_current_tool(Cbs(b"M61 Q-1")).is_err());
        assert_remaining!(set_current_tool(Cbs(b"M61 Q2.5")));
        assert_remaining!(set_current_tool(Cbs(b"M61 Q2.0")));
        assert!(set_current_tool(Cbs(b"M61")).is_err());
    }
}
