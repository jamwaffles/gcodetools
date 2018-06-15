use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub global_move<CompleteByteSlice, GCode>,
    g_code!("53", GCode::GlobalMove)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_global_moves() {
        assert_complete_parse!(global_move(Cbs(b"G53")), GCode::GlobalMove);
    }
}
