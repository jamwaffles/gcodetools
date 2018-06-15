use nom::types::CompleteByteSlice;

use super::super::value::preceded_float_value;
use super::GCode;

named!(pub dwell<CompleteByteSlice, GCode>, map!(
    ws!(preceded!(
        g_code!("4"),
        call!(preceded_float_value, "P")
    )),
    |res| GCode::Dwell(res)
));

#[cfg(test)]
mod tests {
    use super::super::super::value::Value;
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
    fn it_parses_dwells() {
        check_token(dwell(Cbs(b"G04 P10")), GCode::Dwell(Value::Float(10.0)));
        check_token(dwell(Cbs(b"G04 P3")), GCode::Dwell(Value::Float(3.0)));
        check_token(dwell(Cbs(b"G04 P0.5")), GCode::Dwell(Value::Float(0.5)));
        check_token(dwell(Cbs(b"G4 P0.5")), GCode::Dwell(Value::Float(0.5)));
        check_token(dwell(Cbs(b"g4p0.5")), GCode::Dwell(Value::Float(0.5)));
    }
}
