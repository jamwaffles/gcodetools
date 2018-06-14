use nom::types::CompleteByteSlice;

use super::super::value::{preceded_unsigned_value, Value};
use super::GCode;

/// Cutter compensation
#[derive(Debug, PartialEq)]
pub enum CutterCompensation {
    /// No cutter comp (G40)
    Off,
    /// Compensate to the left of the path with optional tool number
    ///
    /// If no tool number is given, the current tool radius is used. If there is no loaded tool,
    /// radius should be 0
    Left(Option<Value>),
    /// Compensate to the right of the path with optional tool number
    ///
    /// If no tool number is given, the current tool radius is used. If there is no loaded tool,
    /// radius should be 0
    Right(Option<Value>),
}

named!(pub cutter_compensation<CompleteByteSlice, GCode>,
    map!(
        alt!(
            g_int!(40, CutterCompensation::Off) |
            map!(
                ws!(preceded!(g_int!(41), opt!(call!(preceded_unsigned_value, "D")))),
                |tool| CutterCompensation::Left(tool)
            ) |
            map!(
                ws!(preceded!(g_int!(42), opt!(call!(preceded_unsigned_value, "D")))),
                |tool| CutterCompensation::Right(tool)
            )
        ),
        |res| GCode::CutterCompensation(res)
    )
);

#[cfg(test)]
mod tests {
    use super::super::super::value::Value;
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_token(
        to_check: Result<(CompleteByteSlice, GCode), nom::Err<CompleteByteSlice>>,
        against: GCode,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_cutter_comp() {
        check_token(
            cutter_compensation(Cbs(b"G40")),
            GCode::CutterCompensation(CutterCompensation::Off),
        );

        check_token(
            cutter_compensation(Cbs(b"G41 D1")),
            GCode::CutterCompensation(CutterCompensation::Left(Some(Value::Unsigned(1u32)))),
        );

        check_token(
            cutter_compensation(Cbs(b"G42 D1")),
            GCode::CutterCompensation(CutterCompensation::Right(Some(Value::Unsigned(1u32)))),
        );

        check_token(
            cutter_compensation(Cbs(b"G42 D0")),
            GCode::CutterCompensation(CutterCompensation::Right(Some(Value::Unsigned(0u32)))),
        );

        check_token(
            cutter_compensation(Cbs(b"G42")),
            GCode::CutterCompensation(CutterCompensation::Right(None)),
        );
    }
}
