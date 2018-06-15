use nom::types::CompleteByteSlice;

use super::GCode;

#[derive(Debug, PartialEq)]
pub enum FeedrateMode {
    InverseTime,
    UnitsPerMinute,
    UnitsPerRevolution,
}

named!(pub feedrate_mode<CompleteByteSlice, GCode>, map!(
    alt!(
        g_code!("93", FeedrateMode::InverseTime) |
        g_code!("94", FeedrateMode::UnitsPerMinute) |
        g_code!("95", FeedrateMode::UnitsPerRevolution)
    ),
    |res| GCode::FeedrateMode(res)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_feedrate_mode() {
        assert_complete_parse!(
            feedrate_mode(Cbs(b"G93")),
            GCode::FeedrateMode(FeedrateMode::InverseTime)
        );
        assert_complete_parse!(
            feedrate_mode(Cbs(b"G94")),
            GCode::FeedrateMode(FeedrateMode::UnitsPerMinute)
        );
        assert_complete_parse!(
            feedrate_mode(Cbs(b"G95")),
            GCode::FeedrateMode(FeedrateMode::UnitsPerRevolution)
        );
    }
}
