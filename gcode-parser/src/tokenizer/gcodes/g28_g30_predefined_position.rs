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
    g_int!(28, GCode::GoToPredefinedPosition(PredefinedPosition::G28)) |
    g_int!(30, GCode::GoToPredefinedPosition(PredefinedPosition::G30)) |
    g_float!(28.1, GCode::StorePredefinedPosition(PredefinedPosition::G28)) |
    g_float!(30.1, GCode::StorePredefinedPosition(PredefinedPosition::G30))
));

#[cfg(test)]
mod tests {
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
    fn it_goes_to_predefined_position() {
        check_token(
            predefined_position(Cbs(b"G28")),
            GCode::GoToPredefinedPosition(PredefinedPosition::G28),
        );
        check_token(
            predefined_position(Cbs(b"G30")),
            GCode::GoToPredefinedPosition(PredefinedPosition::G30),
        );
    }

    #[test]
    fn it_stores_predefined_position() {
        check_token(
            predefined_position(Cbs(b"G28.1")),
            GCode::StorePredefinedPosition(PredefinedPosition::G28),
        );
        check_token(
            predefined_position(Cbs(b"G30.1")),
            GCode::StorePredefinedPosition(PredefinedPosition::G30),
        );
    }
}
