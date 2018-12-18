use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub linear_move<CompleteByteSlice, GCode>,
    g_code!("1", GCode::LinearMove)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_linear_moves() {
        assert_complete_parse!(linear_move(Cbs(b"G1")), GCode::LinearMove);
        assert_complete_parse!(linear_move(Cbs(b"G01")), GCode::LinearMove);
    }
}
