use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub arc<CompleteByteSlice, GCode>, alt!(
    g_code!("2", GCode::ClockwiseArc) |
    g_code!("3", GCode::CounterclockwiseArc)
));

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
    fn it_parses_arcs() {
        check_token(arc(Cbs(b"G2")), GCode::ClockwiseArc);
        check_token(arc(Cbs(b"G02")), GCode::ClockwiseArc);
        check_token(arc(Cbs(b"G3")), GCode::CounterclockwiseArc);
        check_token(arc(Cbs(b"G03")), GCode::CounterclockwiseArc);
    }
}
