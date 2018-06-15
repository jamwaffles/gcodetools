use nom::types::CompleteByteSlice;

use super::GCode;

#[derive(Debug, PartialEq)]
pub enum WorkOffset {
    G54,
    G55,
}

named!(pub work_offset<CompleteByteSlice, GCode>, map!(
    alt!(
        g_code!("54", WorkOffset::G54) |
        g_code!("55", WorkOffset::G55)
    ),
    |res| GCode::WorkOffset(res)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_work_offsets() {
        assert_complete_parse!(work_offset(Cbs(b"G54")), GCode::WorkOffset(WorkOffset::G54));
        assert_complete_parse!(work_offset(Cbs(b"G55")), GCode::WorkOffset(WorkOffset::G55));
    }
}
