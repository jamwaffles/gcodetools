use nom::types::CompleteByteSlice;

use super::helpers::*;
use super::value::*;
use super::{Token, Value};

named!(tool_number<CompleteByteSlice, Token>,
    map!(call!(preceded_unsigned_value, "T"), |res| Token::ToolSelect(res))
);

named!(spindle_speed<CompleteByteSlice, Token>, map!(
    call!(preceded_float_value, "S"),
    |res| {
        let value = match res {
            Value::Float(f) => Value::Signed(f as i32),
            _ => res
        };

        Token::SpindleSpeed(value)
    }
));

named!(feedrate<CompleteByteSlice, Token>, map!(
    call!(preceded_float_value, "F"), |res| Token::FeedRate(res)
));

named!(line_number<CompleteByteSlice, Token>, map!(
    call!(preceded_u32, "N"), |res| Token::LineNumber(res)
));

named!(tool_length_compensation_tool_number<CompleteByteSlice, Token>, map!(
    call!(preceded_unsigned_value, "H"), |res| Token::ToolLengthCompensationToolNumber(res)
));

named!(pub othercode<CompleteByteSlice, Token>,
    alt_complete!(
        tool_number | spindle_speed | feedrate | line_number | tool_length_compensation_tool_number
    )
);

#[cfg(test)]
mod tests {
    use super::super::parameter::Parameter;
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_feed_rate() {
        assert_complete_parse!(
            feedrate(Cbs(b"F100")),
            Token::FeedRate(Value::Float(100.0f32))
        );
        assert_complete_parse!(
            feedrate(Cbs(b"F36.4")),
            Token::FeedRate(Value::Float(36.4f32))
        );
        assert_complete_parse!(
            feedrate(Cbs(b"F-200")),
            Token::FeedRate(Value::Float(-200.0f32))
        );
    }

    #[test]
    fn it_parses_spindle_speed() {
        assert_complete_parse!(
            spindle_speed(Cbs(b"S0")),
            Token::SpindleSpeed(Value::Signed(0i32))
        );
        assert_complete_parse!(
            spindle_speed(Cbs(b"S1000")),
            Token::SpindleSpeed(Value::Signed(1000i32))
        );
        assert_complete_parse!(
            spindle_speed(Cbs(b"S1000.0000")),
            Token::SpindleSpeed(Value::Signed(1000i32))
        );
        assert_complete_parse!(
            spindle_speed(Cbs(b"S-250")),
            Token::SpindleSpeed(Value::Signed(-250i32))
        );
    }

    #[test]
    fn it_parses_tool_number() {
        assert_complete_parse!(
            tool_number(Cbs(b"T0")),
            Token::ToolSelect(Value::Unsigned(0u32))
        );
        assert_complete_parse!(
            tool_number(Cbs(b"T99")),
            Token::ToolSelect(Value::Unsigned(99u32))
        );
    }

    #[test]
    fn it_parses_line_numbers() {
        assert_complete_parse!(line_number(Cbs(b"N10")), Token::LineNumber(10u32));
        assert_complete_parse!(line_number(Cbs(b"N999")), Token::LineNumber(999u32));
    }

    #[test]
    fn it_parses_tool_length_offset_values() {
        assert_complete_parse!(
            tool_length_compensation_tool_number(Cbs(b"H10")),
            Token::ToolLengthCompensationToolNumber(Value::Unsigned(10u32))
        );
        assert_complete_parse!(
            tool_length_compensation_tool_number(Cbs(b"H0")),
            Token::ToolLengthCompensationToolNumber(Value::Unsigned(0u32))
        );
    }

    #[test]
    fn it_parses_parameterised_feed() {
        assert_complete_parse!(
            feedrate(Cbs(b"f #<feedrate>")),
            Token::FeedRate(Value::Parameter(Parameter::Named("feedrate".into())))
        );
    }
}
