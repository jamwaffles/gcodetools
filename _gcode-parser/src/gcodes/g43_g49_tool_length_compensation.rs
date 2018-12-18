use nom::types::CompleteByteSlice;

use super::super::vec9::{vec9, Vec9};
use super::GCode;

/// Tool length compensation
#[derive(Debug, PartialEq)]
pub enum ToolLengthCompensation {
    /// No tool length compensation
    Disable,
    /// Use offset from tool number (defaults to currently loaded tool tool)
    // TODO: Tool number here
    ToolNumberOffset,
    /// Apply a vector offset to any existing offsets
    Dynamic(Vec9),
}

named!(pub tool_length_compensation<CompleteByteSlice, GCode>, map!(
    alt!(
        g_code!("43", ToolLengthCompensation::ToolNumberOffset) |
        map!(
            ws!(preceded!(g_code!("43.1"), vec9)),
            |offset| ToolLengthCompensation::Dynamic(offset)
        ) |
        g_code!("49", ToolLengthCompensation::Disable)
    ),
    |res| GCode::ToolLengthCompensation(res)
));

#[cfg(test)]
mod tests {
    use super::super::super::value::Value;
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_dynamic_tool_length_offset() {
        assert_complete_parse!(
            tool_length_compensation(Cbs(b"G43.1 Z0.250")),
            GCode::ToolLengthCompensation(ToolLengthCompensation::Dynamic(Vec9 {
                z: Some(Value::Float(0.250)),
                ..Default::default()
            }))
        );
    }
}
