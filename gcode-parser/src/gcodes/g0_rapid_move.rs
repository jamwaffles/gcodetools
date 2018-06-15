use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub rapid_move<CompleteByteSlice, GCode>,
    g_code!("0", GCode::RapidMove)
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
    fn it_parses_rapids() {
        check_token(rapid_move(Cbs(b"G0")), GCode::RapidMove);
        check_token(rapid_move(Cbs(b"G00")), GCode::RapidMove);
    }
}
