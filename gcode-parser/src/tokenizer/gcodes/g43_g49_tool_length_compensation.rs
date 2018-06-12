use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::vec9::{vec9, Vec9};
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum ToolLengthCompensation {
    Disable,
    // TODO: Tool number here
    ToolNumberOffset,
    Dynamic(Vec9),
}

named!(pub tool_length_compensation<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 43.0), |_| ToolLengthCompensation::ToolNumberOffset) |
        map!(
            ws!(preceded!(call!(g, 43.1), vec9)),
            |offset| ToolLengthCompensation::Dynamic(offset)
        ) |
        map!(call!(g, 49.0), |_| ToolLengthCompensation::Disable)
    ),
    |res| Token::ToolLengthCompensation(res)
));

#[cfg(test)]
mod tests {
    use super::super::super::value::Value;
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
    fn it_parses_dynamic_tool_length_offset() {
        check_token(
            tool_length_compensation(Cbs(b"G43.1 Z0.250")),
            Token::ToolLengthCompensation(ToolLengthCompensation::Dynamic(Vec9 {
                z: Some(Value::Float(0.250)),
                ..Default::default()
            })),
        );
    }
}
