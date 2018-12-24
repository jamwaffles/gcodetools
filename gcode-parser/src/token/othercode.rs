use crate::parsers::code_number;
use crate::Span;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq, Clone)]
pub struct Feedrate<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) feedrate: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpindleSpeed<'a> {
    pub(crate) span: Span<'a>,

    // Spindle speed value in revolutions per minute (RPM)
    //
    // This value cannot be negative. Reverse rotation is achieved by issuing an `M4 Sxxxx` command
    pub(crate) rpm: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ToolNumber<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) tool_number: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OtherCode<'a> {
    Feedrate(Feedrate<'a>),
    SpindleSpeed(SpindleSpeed<'a>),
    ToolNumber(ToolNumber<'a>),
}

named!(pub feedrate<Span, Feedrate>,
    do_parse!(
        span: position!() >>
        feedrate: preceded!(tag_no_case!("F"), code_number) >>
        (Feedrate { span, feedrate })
    )
);

named!(pub spindle_speed<Span, SpindleSpeed>,
    do_parse!(
        span: position!() >>
        rpm: preceded!(
            tag_no_case!("S"),
            flat_map!(
                digit1,
                parse_to!(u32)
            )
        ) >>
        (SpindleSpeed { span, rpm })
    )
);

named!(pub tool_number<Span, ToolNumber>,
    do_parse!(
        span: position!() >>
        tool_number: preceded!(
            tag_no_case!("T"),
            flat_map!(
                digit1,
                parse_to!(u16)
            )
        ) >>
        (ToolNumber { span, tool_number })
    )
);

named!(pub othercode<Span, OtherCode>,
    alt_complete!(
        map!(feedrate, OtherCode::Feedrate) |
        map!(spindle_speed, OtherCode::SpindleSpeed) |
        map!(tool_number, OtherCode::ToolNumber)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_feedrate() {
        assert_parse!(
            parser = othercode,
            input = span!(b"F500.3"),
            expected = OtherCode::Feedrate(Feedrate {
                feedrate: 500.3,
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 6)
        );
    }

    #[test]
    fn parse_spindle_rpm() {
        assert_parse!(
            parser = othercode,
            input = span!(b"S1000"),
            expected = OtherCode::SpindleSpeed(SpindleSpeed {
                rpm: 1000u32,
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 5)
        );
    }

    #[test]
    fn parse_tool_number() {
        assert_parse!(
            parser = othercode,
            input = span!(b"T32"),
            expected = OtherCode::ToolNumber(ToolNumber {
                tool_number: 32u16,
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 3)
        );
    }
}
