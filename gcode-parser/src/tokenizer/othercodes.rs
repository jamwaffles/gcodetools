use nom::types::CompleteByteSlice;

use super::Token;
use super::helpers::*;

named!(pub tool_number<CompleteByteSlice, Token>,
    map!(call!(preceded_u32, "T"), |res| Token::ToolSelect(res))
);

named!(pub spindle_speed<CompleteByteSlice, Token>, map!(
    call!(preceded_i32, "S"), |res| Token::SpindleSpeed(res)
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
    fn it_parses_spindle_speed() {
        check_token(spindle_speed(Cbs(b"S0")), Token::SpindleSpeed(0i32));
        check_token(spindle_speed(Cbs(b"S1000")), Token::SpindleSpeed(1000i32));
        check_token(spindle_speed(Cbs(b"S-250")), Token::SpindleSpeed(-250i32));
    }

    #[test]
    fn it_parses_tool_number() {
        check_token(tool_number(Cbs(b"T0")), Token::ToolSelect(0u32));
        check_token(tool_number(Cbs(b"T99")), Token::ToolSelect(99u32));
    }
}
