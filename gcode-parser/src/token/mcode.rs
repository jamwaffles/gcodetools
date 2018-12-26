use crate::Span;
use nom::*;
use nom_locate::position;

/// An M-code
#[derive(Debug, PartialEq, Clone)]
pub enum MCode<'a> {
    /// Turn the spindle clockwise
    SpindleForward(SpindleForward<'a>),

    /// Turn the spindle counter-clockwise
    SpindleReverse(SpindleReverse<'a>),

    /// Stop the spindle
    SpindleStop(SpindleStop<'a>),

    /// Change tool
    ToolChange(ToolChange<'a>),
}

/// Start the spindle spinning in a clockwise direction
#[derive(Debug, PartialEq, Clone)]
pub struct SpindleForward<'a> {
    /// Position in source input
    pub span: Span<'a>,
}

/// Start the spindle spinning in a clockwise direction
#[derive(Debug, PartialEq, Clone)]
pub struct SpindleReverse<'a> {
    /// Position in source input
    pub span: Span<'a>,
}

/// Start the spindle spinning in a clockwise direction
#[derive(Debug, PartialEq, Clone)]
pub struct SpindleStop<'a> {
    /// Position in source input
    pub span: Span<'a>,
}

/// Start the spindle spinning in a clockwise direction
#[derive(Debug, PartialEq, Clone)]
pub struct ToolChange<'a> {
    /// Position in source input
    pub span: Span<'a>,
}

named!(pub mcode<Span, MCode>,
    // TODO: Handle leading zeros like `M06`, etc
    alt_complete!(
        positioned!(tag_no_case!("M3"), |(span, _)| MCode::SpindleForward(SpindleForward { span })) |
        positioned!(tag_no_case!("M4"), |(span, _)| MCode::SpindleReverse(SpindleReverse { span })) |
        positioned!(tag_no_case!("M5"), |(span, _)| MCode::SpindleStop(SpindleStop { span })) |
        positioned!(tag_no_case!("M6"), |(span, _)| MCode::ToolChange(ToolChange { span }))
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
            expected = MCode::SpindleForward(SpindleForward {
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 2)
        );

        assert_parse!(
            parser = mcode,
            input = span!(b"M4"),
            expected = MCode::SpindleReverse(SpindleReverse {
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 2)
        );

        assert_parse!(
            parser = mcode,
            input = span!(b"M5"),
            expected = MCode::SpindleStop(SpindleStop {
                span: empty_span!()
            }),
            remaining = empty_span!(offset = 2)
        );
    }
}
