use nom::types::CompleteByteSlice;

use super::MCode;

named!(pub pause<CompleteByteSlice, MCode>, alt!(
    m_int!(0, MCode::Pause) |
    m_int!(1, MCode::OptionalPause)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_token(
        to_check: Result<(CompleteByteSlice, MCode), nom::Err<CompleteByteSlice>>,
        against: MCode,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_pauses() {
        check_token(pause(Cbs(b"M0")), MCode::Pause);
        check_token(pause(Cbs(b"M00")), MCode::Pause);

        check_token(pause(Cbs(b"M1")), MCode::OptionalPause);
        check_token(pause(Cbs(b"M01")), MCode::OptionalPause);
    }
}
