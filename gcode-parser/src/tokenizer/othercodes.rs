use nom::types::CompleteByteSlice;

use super::Token;
use super::helpers::*;

named!(tool_number<CompleteByteSlice, Token>,
    map!(call!(preceded_u32, "T"), |res| Token::ToolSelect(res))
);

named!(spindle_speed<CompleteByteSlice, Token>, map!(
    call!(preceded_i32, "S"), |res| Token::SpindleSpeed(res)
));

named!(feedrate<CompleteByteSlice, Token>, map!(
    call!(preceded_f32, "F"), |res| Token::FeedRate(res)
));

named!(pub othercode<CompleteByteSlice, Token>,
    alt_complete!(
        tool_number | spindle_speed | feedrate
    )
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
    fn it_parses_feed_rate() {
        check_token(feedrate(Cbs(b"F100")), Token::FeedRate(100.0f32));
        check_token(feedrate(Cbs(b"F36.4")), Token::FeedRate(36.4f32));
        check_token(feedrate(Cbs(b"F-200")), Token::FeedRate(-200.0f32));
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
