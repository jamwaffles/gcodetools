use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub global_move<CompleteByteSlice, GCode>,
    g_code!("53", GCode::GlobalMove)
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
    fn it_parses_global_moves() {
        check_token(global_move(Cbs(b"G53")), GCode::GlobalMove);
    }
}
