use crate::{map_code, Span};
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
        map_code!("M3", |_| MCode::SpindleForward) |
        map_code!("M4", |_| MCode::SpindleReverse) |
        map_code!("M5", |_| MCode::SpindleStop) |
        map_code!("M6", |_| MCode::ToolChange)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spindle_commands() {
        assert_parse!(
            parser = mcode;
            input = span!(b"M3");
            expected = MCode::SpindleForward
        );

        assert_parse!(
            parser = mcode;
            input = span!(b"M4");
            expected = MCode::SpindleReverse
        );

        assert_parse!(
            parser = mcode;
            input = span!(b"M5");
            expected = MCode::SpindleStop
        );
    }
}
