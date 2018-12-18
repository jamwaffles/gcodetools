use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub rapid_move<CompleteByteSlice, GCode>,
    g_code!("0", GCode::RapidMove)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_rapids() {
        assert_complete_parse!(rapid_move(Cbs(b"G0")), GCode::RapidMove);
        assert_complete_parse!(rapid_move(Cbs(b"G00")), GCode::RapidMove);
    }
}
