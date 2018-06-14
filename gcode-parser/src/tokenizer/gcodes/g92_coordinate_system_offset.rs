use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub coordinate_system_offset<CompleteByteSlice, GCode>, alt!(
    g_float!(92.0, GCode::CoordinateSystemOffset) |
    g_float!(92.1, GCode::CoordinateSystemOffsetHardReset) |
    g_float!(92.2, GCode::CoordinateSystemOffsetSoftReset)
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
    fn it_parses_coord_system_hard_resets() {
        check_token(
            coordinate_system_offset(Cbs(b"G92.1")),
            GCode::CoordinateSystemOffsetHardReset,
        );
    }

    #[test]
    fn it_parses_coord_system_soft_resets() {
        check_token(
            coordinate_system_offset(Cbs(b"G92.2")),
            GCode::CoordinateSystemOffsetSoftReset,
        );
    }
}
