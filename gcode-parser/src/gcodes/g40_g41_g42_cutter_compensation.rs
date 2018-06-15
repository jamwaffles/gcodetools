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
            g_code!("40", CutterCompensation::Off) |
            map!(
                ws!(preceded!(g_code!("41"), opt!(call!(preceded_unsigned_value, "D")))),
                |tool| CutterCompensation::Left(tool)
            ) |
            map!(
                ws!(preceded!(g_code!("42"), opt!(call!(preceded_unsigned_value, "D")))),
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
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_cutter_comp() {
        assert_complete_parse!(
            cutter_compensation(Cbs(b"G40")),
            GCode::CutterCompensation(CutterCompensation::Off)
        );

        assert_complete_parse!(
            cutter_compensation(Cbs(b"G41 D1")),
            GCode::CutterCompensation(CutterCompensation::Left(Some(Value::Unsigned(1u32))))
        );

        assert_complete_parse!(
            cutter_compensation(Cbs(b"G42 D1")),
            GCode::CutterCompensation(CutterCompensation::Right(Some(Value::Unsigned(1u32))))
        );

        assert_complete_parse!(
            cutter_compensation(Cbs(b"G42 D0")),
            GCode::CutterCompensation(CutterCompensation::Right(Some(Value::Unsigned(0u32))))
        );

        assert_complete_parse!(
            cutter_compensation(Cbs(b"G42")),
            GCode::CutterCompensation(CutterCompensation::Right(None))
        );
    }
}
