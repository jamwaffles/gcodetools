use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub coordinate_system_offset<CompleteByteSlice, GCode>, alt!(
    g_code!("92", GCode::CoordinateSystemOffset) |
    g_code!("92.1", GCode::CoordinateSystemOffsetHardReset) |
    g_code!("92.2", GCode::CoordinateSystemOffsetSoftReset)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_coord_system_hard_resets() {
        assert_complete_parse!(
            coordinate_system_offset(Cbs(b"G92.1")),
            GCode::CoordinateSystemOffsetHardReset
        );
    }

    #[test]
    fn it_parses_coord_system_soft_resets() {
        assert_complete_parse!(
            coordinate_system_offset(Cbs(b"G92.2")),
            GCode::CoordinateSystemOffsetSoftReset
        );
    }
}
