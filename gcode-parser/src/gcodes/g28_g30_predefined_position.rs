use nom::types::CompleteByteSlice;

use super::GCode;

/// One of two predefined positions
///
/// Used to select which position to refer to when storing/traversing to it
#[derive(Debug, PartialEq)]
pub enum PredefinedPosition {
    /// First one (G28)
    G28,
    /// Second one (G30)
    G30,
}

named!(pub predefined_position<CompleteByteSlice, GCode>, alt!(
    g_code!("28", GCode::GoToPredefinedPosition(PredefinedPosition::G28)) |
    g_code!("30", GCode::GoToPredefinedPosition(PredefinedPosition::G30)) |
    g_code!("28.1", GCode::StorePredefinedPosition(PredefinedPosition::G28)) |
    g_code!("30.1", GCode::StorePredefinedPosition(PredefinedPosition::G30))
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_goes_to_predefined_position() {
        assert_complete_parse!(
            predefined_position(Cbs(b"G28")),
            GCode::GoToPredefinedPosition(PredefinedPosition::G28)
        );
        assert_complete_parse!(
            predefined_position(Cbs(b"G30")),
            GCode::GoToPredefinedPosition(PredefinedPosition::G30)
        );
    }

    #[test]
    fn it_stores_predefined_position() {
        assert_complete_parse!(
            predefined_position(Cbs(b"G28.1")),
            GCode::StorePredefinedPosition(PredefinedPosition::G28)
        );
        assert_complete_parse!(
            predefined_position(Cbs(b"G30.1")),
            GCode::StorePredefinedPosition(PredefinedPosition::G30)
        );
    }
}
