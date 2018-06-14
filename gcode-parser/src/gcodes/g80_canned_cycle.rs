use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub canned_cycle<CompleteByteSlice, GCode>,
    g_int!(80, GCode::CancelCannedCycle)
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
    fn it_parses_canned_cycles() {
        check_token(canned_cycle(Cbs(b"G80")), GCode::CancelCannedCycle);
    }
}
