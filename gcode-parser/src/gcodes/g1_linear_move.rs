use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub linear_move<CompleteByteSlice, GCode>,
    g_int!(1, GCode::LinearMove)
);

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
    fn it_parses_linear_moves() {
        check_token(linear_move(Cbs(b"G1")), GCode::LinearMove);
        check_token(linear_move(Cbs(b"G01")), GCode::LinearMove);
    }
}
