use common::parsing::Span;
use expression::{
    parser::{ngc_float_value, ngc_unsigned_value},
    Value,
};
use nom::*;

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

named!(pub(crate) feedrate<Span, Feedrate>,
    map!(
        sep!(
            space0,
            preceded!(char_no_case!('F'), ngc_float_value)
        ),
        |feedrate| Feedrate { feedrate }
    )
);

named!(pub(crate) spindle_speed<Span, SpindleSpeed>,
    map!(
        sep!(
            space0,
            preceded!(char_no_case!('S'), ngc_float_value)
        ),
        |rpm| SpindleSpeed { rpm }
    )
);

named!(pub tool_number<Span, ToolNumber>,
    map!(
        sep!(
            space0,
            preceded!(char_no_case!('T'), ngc_unsigned_value)
        ),
        |tool_number| ToolNumber { tool_number }
    )
);

named!(pub line_number<Span, LineNumber>,
    map!(
        sep!(
            space0,
            preceded!(
                char_no_case!('N'),
                flat_map!(
                    digit1,
                    parse_to!(u32)
                )
            )
        ),
        |line_number| LineNumber { line_number }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};
    use expression::Parameter;

    #[test]
    fn parse_feedrate() {
        assert_parse!(
            parser = feedrate;
            input = span!(b"F500.3");
            expected = Feedrate { feedrate: Value::Float(500.3) }
        );
    }

    #[test]
    fn parse_spindle_rpm() {
        assert_parse!(
            parser = spindle_speed;
            input = span!(b"S1000");
            expected = SpindleSpeed { rpm: Value::Float(1000.0) }
        );

        assert_parse!(
            parser = spindle_speed;
            input = span!(b"S1234.5678");
            expected = SpindleSpeed { rpm: Value::Float(1234.5678) }
        );
    }

    #[test]
    fn parse_tool_number() {
        assert_parse!(
            parser = tool_number;
            input = span!(b"T32");
            expected = ToolNumber { tool_number: Value::Unsigned(32) }
        );
    }

    #[test]
    fn parse_line_number() {
        assert_parse!(
            parser = line_number;
            input = span!(b"N1234");
            expected = LineNumber {
                line_number: 1234u32
            }
        );
    }

    #[test]
    fn parse_no_trailing_number() {
        assert_parse!(
            parser = feedrate;
            input = span!(b"F5.");
            expected = Feedrate { feedrate: Value::Float(5.0) }
        );
    }

    #[test]
    fn parse_space_sep() {
        assert_parse!(
            parser = feedrate;
            input = span!(b"f #<feedrate>");
            expected = Feedrate { feedrate: Value::Parameter(Parameter::Named("feedrate".to_string())) }
        );
    }
}
