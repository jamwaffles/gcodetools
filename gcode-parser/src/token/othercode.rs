use crate::parsers::code_number;
use crate::Span;
use nom::*;

/// Define a feed rate in machine units per minute
#[derive(Debug, PartialEq, Clone)]
pub struct Feedrate {
    /// Feed rate in machine units per minute
    pub feedrate: f32,
}

/// Spindle speed value
#[derive(Debug, PartialEq, Clone)]
pub struct SpindleSpeed {
    /// Spindle speed value in revolutions per minute (RPM)
    ///
    /// This value cannot be negative. Reverse rotation is achieved by issuing an `M4 Sxxxx` command
    pub rpm: f32,
}

/// Tool number `Tn`
#[derive(Debug, PartialEq, Clone)]
pub struct ToolNumber {
    /// Positive integer tool number
    pub tool_number: u16,
}

/// Line number `Nn`
#[derive(Debug, PartialEq, Clone)]
pub struct LineNumber {
    /// Positive integer line number
    pub line_number: u32,
}

named!(pub(crate) feedrate<Span, Feedrate>,
    map!(
        preceded!(char_no_case!('F'), code_number),
        |feedrate| Feedrate { feedrate }
    )
);

named!(pub(crate) spindle_speed<Span, SpindleSpeed>,
    map!(
        preceded!(
            char_no_case!('S'),
            float
        ),
        |rpm| SpindleSpeed { rpm }
    )
);

named!(pub tool_number<Span, ToolNumber>,
    map!(
        preceded!(
            char_no_case!('T'),
            flat_map!(
                digit1,
                parse_to!(u16)
            )
        ),
        |tool_number| ToolNumber { tool_number }
    )
);

named!(pub line_number<Span, LineNumber>,
    map!(
        preceded!(
            char_no_case!('N'),
            flat_map!(
                digit1,
                parse_to!(u32)
            )
        ),
        |line_number| LineNumber { line_number }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_feedrate() {
        assert_parse!(
            parser = feedrate,
            input = span!(b"F500.3"),
            expected = Feedrate { feedrate: 500.3 }
        );
    }

    #[test]
    fn parse_spindle_rpm() {
        assert_parse!(
            parser = spindle_speed,
            input = span!(b"S1000"),
            expected = SpindleSpeed { rpm: 1000.0f32 }
        );

        assert_parse!(
            parser = spindle_speed,
            input = span!(b"S1234.5678"),
            expected = SpindleSpeed { rpm: 1234.5678f32 }
        );
    }

    #[test]
    fn parse_tool_number() {
        assert_parse!(
            parser = tool_number,
            input = span!(b"T32"),
            expected = ToolNumber { tool_number: 32u16 }
        );
    }

    #[test]
    fn parse_line_number() {
        assert_parse!(
            parser = line_number,
            input = span!(b"N1234"),
            expected = LineNumber {
                line_number: 1234u32
            }
        );
    }
}
