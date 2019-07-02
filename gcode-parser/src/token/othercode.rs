use crate::value::{value, Value};
use nom::{
    bytes::streaming::tag_no_case,
    character::streaming::digit1,
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
    pub tool_number: Value,
}

/// Line number `Nn`
#[derive(Debug, PartialEq, Clone)]
pub struct LineNumber {
    /// Positive integer line number
    pub line_number: u32,
}

// named!(pub(crate) feedrate<Span, Feedrate>,
//     map!(
//         sep!(
//             space0,
//             preceded!(char_no_case!('F'), ngc_float_value)
//         ),
//         |feedrate| Feedrate { feedrate }
//     )
// );

pub fn feedrate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Feedrate, E> {
    context(
        "feed rate",
        map(preceded(tag_no_case("F"), value), |feedrate| Feedrate {
            feedrate,
        }),
    )(i)
}

// named!(pub(crate) spindle_speed<Span, SpindleSpeed>,
//     map!(
//         sep!(
//             space0,
//             preceded!(char_no_case!('S'), ngc_float_value)
//         ),
//         |rpm| SpindleSpeed { rpm }
//     )
// );

pub fn spindle_speed<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, SpindleSpeed, E> {
    context(
        "spindle speed",
        map(preceded(tag_no_case("S"), value), |rpm| SpindleSpeed {
            rpm,
        }),
    )(i)
}

// named!(pub tool_number<Span, ToolNumber>,
//     map!(
//         sep!(
//             space0,
//             preceded!(char_no_case!('T'), ngc_unsigned_value)
//         ),
//         |tool_number| ToolNumber { tool_number }
//     )
// );

pub fn tool_number<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ToolNumber, E> {
    // TODO: Parse to unsigned int
    context(
        "tool number",
        map(preceded(tag_no_case("T"), value), |tool_number| {
            ToolNumber { tool_number }
        }),
    )(i)
}

// named!(pub raw_line_number<Span, LineNumber>,
//     map!(
//         sep!(
//             space0,
//             preceded!(
//                 char_no_case!('N'),
//                 flat_map!(
//                     digit1,
//                     parse_to!(u32)
//                 )
//             )
//         ),
//         |line_number| LineNumber { line_number }
//     )
// );

pub fn raw_line_number<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, LineNumber, E> {
    context(
        "line number",
        map(
            preceded(
                tag_no_case("N"),
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

    #[test]
    fn parse_feedrate() {
        assert_parse!(
            parser = feedrate;
            input = "F500.3";
            expected = Feedrate { feedrate: 500.3.into() }
        );
    }

    #[test]
    fn parse_spindle_rpm() {
        assert_parse!(
            parser = spindle_speed;
            input = "S1000";
            expected = SpindleSpeed { rpm: 1000.0.into() }
        );

        assert_parse!(
            parser = spindle_speed;
            input = "S1234.5678";
            expected = SpindleSpeed { rpm: 1234.5678.into() }
        );
    }

    #[test]
    fn parse_tool_number() {
        assert_parse!(
            parser = tool_number;
            input = "T32";
            // TODO: Parse to unsigned int
            expected = ToolNumber { tool_number: 32.0.into() }
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

    // TODO: Re-enable
    // #[test]
    // fn parse_space_sep() {
    //     assert_parse!(
    //         parser = feedrate;
    //         input = "f #<feedrate>";
    //         expected = Feedrate { feedrate: Value::Parameter(Parameter::Named("feedrate".to_string())) }
    //     );
    // }
}
