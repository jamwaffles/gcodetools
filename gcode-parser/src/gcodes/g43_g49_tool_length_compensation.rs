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
        g_int!(43, ToolLengthCompensation::ToolNumberOffset) |
        map!(
            ws!(preceded!(g_float!(43.1), vec9)),
            |offset| ToolLengthCompensation::Dynamic(offset)
        ) |
        g_int!(49, ToolLengthCompensation::Disable)
    ),
    |res| GCode::ToolLengthCompensation(res)
));

#[cfg(test)]
mod tests {
    use super::super::super::value::Value;
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_token(
        to_check: Result<(CompleteByteSlice, GCode), nom::Err<CompleteByteSlice>>,
        against: GCode,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_dynamic_tool_length_offset() {
        check_token(
            tool_length_compensation(Cbs(b"G43.1 Z0.250")),
            GCode::ToolLengthCompensation(ToolLengthCompensation::Dynamic(Vec9 {
                z: Some(Value::Float(0.250)),
                ..Default::default()
            })),
        );
    }
}
