use crate::Span;
use nom::*;

/// An M-code
#[derive(Debug, PartialEq, Clone)]
pub enum MCode {
    /// Turn the spindle clockwise
    SpindleForward,

    /// Turn the spindle counter-clockwise
    SpindleReverse,

    /// Stop the spindle
    SpindleStop,

    /// Change tool
    ToolChange,
}

named!(pub mcode<Span, MCode>,
    // TODO: Handle leading zeros like `M06`, etc
    alt_complete!(
        map!(tag_no_case!("M3"), |_| MCode::SpindleForward) |
        map!(tag_no_case!("M4"), |_| MCode::SpindleReverse) |
        map!(tag_no_case!("M5"), |_| MCode::SpindleStop) |
        map!(tag_no_case!("M6"), |_| MCode::ToolChange)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spindle_commands() {
        assert_parse!(
            parser = mcode,
            input = span!(b"M3"),
            expected = MCode::SpindleForward,
            remaining = empty_span!(offset = 2)
        );

        assert_parse!(
            parser = mcode,
            input = span!(b"M4"),
            expected = MCode::SpindleReverse,
            remaining = empty_span!(offset = 2)
        );

        assert_parse!(
            parser = mcode,
            input = span!(b"M5"),
            expected = MCode::SpindleStop,
            remaining = empty_span!(offset = 2)
        );
    }
}
