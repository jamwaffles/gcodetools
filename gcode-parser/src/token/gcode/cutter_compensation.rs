use crate::map_code;
use common::parsing::Span;
use expression::{parser::ngc_float_value, Value};
use nom::*;

/// Cutter compensation type
#[derive(Debug, PartialEq, Clone)]
pub enum CutterCompensation {
    /// Turn cutter compensation off (G40)
    Off,

    /// Offset the tool to the left of the path (G41)
    Left(Option<Value>),

    /// Offset the tool to the right of the path (G42)
    Right(Option<Value>),
}

named!(pub cutter_compensation<Span, CutterCompensation>,
    alt_complete!(
        map_code!(
            "G40",
            |_| CutterCompensation::Off
        ) |
        map_code!(
            "G41",
            opt!(
                preceded!(
                    char_no_case!('D'),
                    ngc_float_value
                )
            ),
            |dia| CutterCompensation::Left(dia)
        ) |
        map_code!(
            "G42",
            opt!(
                preceded!(
                    char_no_case!('D'),
                    ngc_float_value
                )
            ),
            |dia| CutterCompensation::Right(dia)
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};

    #[test]
    fn parse_cutter_compensation() {
        assert_parse!(
            parser = cutter_compensation;
            input =
                span!(b"G40"),
                span!(b"G41"),
                span!(b"G42")
            ;
            expected =
                CutterCompensation::Off,
                CutterCompensation::Left(None),
                CutterCompensation::Right(None)
            ;
        );
    }

    #[test]
    fn parse_offsets() {
        assert_parse!(
            parser = cutter_compensation;
            input =
                span!(b"G41 D5.0"),
                span!(b"G42d10.1")
            ;
            expected =
                CutterCompensation::Left(Some(5.0.into())),
                CutterCompensation::Right(Some(10.1.into()))
            ;
        );
    }
}
