use nom::types::CompleteByteSlice;

use super::GCode;

#[derive(Debug, PartialEq)]
pub enum DistanceMode {
    Absolute,
    Incremental,
}

named!(pub distance_mode<CompleteByteSlice, GCode>, map!(
    alt!(
        g_code!("90", DistanceMode::Absolute) |
        g_code!("91", DistanceMode::Incremental)
    ),
    |res| GCode::DistanceMode(res)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_distance_mode() {
        assert_complete_parse!(
            distance_mode(Cbs(b"G90")),
            GCode::DistanceMode(DistanceMode::Absolute)
        );

        assert_complete_parse!(
            distance_mode(Cbs(b"G91")),
            GCode::DistanceMode(DistanceMode::Incremental)
        );
    }
}
