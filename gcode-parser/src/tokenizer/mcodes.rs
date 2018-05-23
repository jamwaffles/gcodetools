use nom::types::CompleteByteSlice;

use super::Token;

#[derive(Debug, PartialEq)]
pub enum SpindleRotation {
    Cw,
    Ccw,
    Stop,
}

named!(pub tool_change<CompleteByteSlice, Token>,
    map!(tag!("M6"), |_| Token::ToolChange)
);

named!(pub spindle_rotation<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("M3"), |_| SpindleRotation::Cw) |
        map!(tag_no_case!("M4"), |_| SpindleRotation::Ccw) |
        map!(tag_no_case!("M5"), |_| SpindleRotation::Stop)
    ),
    |res| Token::SpindleRotation(res)
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
    fn it_parses_spindle_rotation() {
        check_token(
            spindle_rotation(Cbs(b"M3")),
            Token::SpindleRotation(SpindleRotation::Cw),
        );
        check_token(
            spindle_rotation(Cbs(b"M4")),
            Token::SpindleRotation(SpindleRotation::Ccw),
        );
        check_token(
            spindle_rotation(Cbs(b"M5")),
            Token::SpindleRotation(SpindleRotation::Stop),
        );
    }

    #[test]
    fn it_changes_tool() {
        check_token(tool_change(Cbs(b"M6")), Token::ToolChange);
    }
}
