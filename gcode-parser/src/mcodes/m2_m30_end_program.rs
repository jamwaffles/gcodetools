use nom::types::CompleteByteSlice;

use super::MCode;

named!(pub end_program<CompleteByteSlice, MCode>, alt!(
    m_code!("30", MCode::EndProgram) |
    m_code!("2", MCode::EndProgram)
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
    fn it_parses_end_program() {
        check_token(end_program(Cbs(b"M2")), MCode::EndProgram);
        check_token(end_program(Cbs(b"M30")), MCode::EndProgram);
    }
}
