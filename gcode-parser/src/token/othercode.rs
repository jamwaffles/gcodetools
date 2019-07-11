use crate::parsers::char_no_case;
use crate::value::{
    preceded_positive_decimal_value, preceded_unsigned_value, UnsignedValue, Value,
};
use nom::{
    character::complete::digit1,
    combinator::{map, map_res},
    error::{context, ParseError},
    sequence::preceded,
    IResult,
};

/// Define a feed rate in machine units per minute
#[derive(Debug, PartialEq, Clone)]
pub struct Feedrate {
    /// Feed rate in machine units per minute
    pub feedrate: Value,
}

/// Spindle speed value
#[derive(Debug, PartialEq, Clone)]
pub struct SpindleSpeed {
    /// Spindle speed value in revolutions per minute (RPM)
    ///
    /// This value cannot be negative. Reverse rotation is achieved by issuing an `M4 Sxxxx` command
    pub rpm: Value,
}

/// Tool number `Tn`
#[derive(Debug, PartialEq, Clone)]
pub struct ToolNumber {
    /// Positive integer tool number
    pub tool_number: UnsignedValue,
}

/// Line number `Nn`
#[derive(Debug, PartialEq, Clone)]
pub struct LineNumber {
    /// Positive integer line number
    pub line_number: u32,
}

pub fn feedrate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Feedrate, E> {
    context(
        "feed rate",
        map(
            preceded_positive_decimal_value(char_no_case('F')),
            |feedrate| Feedrate { feedrate },
        ),
    )(i)
}

pub fn spindle_speed<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, SpindleSpeed, E> {
    context(
        "spindle speed",
        map(preceded_positive_decimal_value(char_no_case('S')), |rpm| {
            SpindleSpeed { rpm }
        }),
    )(i)
}

pub fn tool_number<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ToolNumber, E> {
    context(
        "tool number",
        map(preceded_unsigned_value(char_no_case('T')), |tool_number| {
            ToolNumber { tool_number }
        }),
    )(i)
}

pub fn raw_line_number<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, LineNumber, E> {
    context(
        "line number",
        map(
            preceded(
                char_no_case('N'),
                map_res(digit1, |n: &'a str| n.parse::<u32>()),
            ),
            |line_number| LineNumber { line_number },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use expression::Parameter;

    #[test]
    fn parse_feedrate_decimal() {
        assert_parse!(
            parser = feedrate;
            input = "F500.3";
            expected = Feedrate { feedrate: 500.3.into() }
        );
    }

    #[test]
    fn parse_feedrate() {
        assert_parse!(
            parser = feedrate;
            input = "f500";
            expected = Feedrate { feedrate: 500.0.into() }
        );
    }

    #[test]
    fn parse_spindle_rpm() {
        assert_parse!(
            parser = spindle_speed;
            input = "S1000";
            expected = SpindleSpeed { rpm: Value::Literal(1000.0.into()) }
        );

        assert_parse!(
            parser = spindle_speed;
            input = "S1234.5678";
            expected = SpindleSpeed { rpm: Value::Literal(1234.5678).into() }
        );
    }

    #[test]
    fn parse_tool_number() {
        assert_parse!(
            parser = tool_number;
            input = "T32";
            expected = ToolNumber { tool_number: 32.into() }
        );
    }

    #[test]
    fn parse_line_number() {
        assert_parse!(
            parser = raw_line_number;
            input = "N1234";
            expected = LineNumber {
                line_number: 1234u32
            }
        );
    }

    #[test]
    fn parse_no_trailing_number() {
        assert_parse!(
            parser = feedrate;
            input = "F5.";
            expected = Feedrate { feedrate: 5.0.into() }
        );
    }

    #[test]
    fn parse_space_sep_spindle() {
        assert_parse!(
            parser = spindle_speed;
            input = "s #<rpm> m3 (spindle cw)";
            expected = SpindleSpeed { rpm: Value::Parameter(Parameter::Local("rpm".to_string())) };
            remaining = " m3 (spindle cw)"
        );
    }

    #[test]
    fn parse_space_sep() {
        assert_parse!(
            parser = feedrate;
            input = "f #<feedrate>";
            expected = Feedrate { feedrate: Value::Parameter(Parameter::Local("feedrate".to_string())) }
        );
    }
}
