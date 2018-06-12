use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::value::preceded_float_value;
use super::super::Token;

named!(pub dwell<CompleteByteSlice, Token>, map!(
    ws!(preceded!(
        call!(g, 4.0),
        call!(preceded_float_value, "P")
    )),
    |res| Token::Dwell(res)
));

#[cfg(test)]
mod tests {
    use super::super::super::value::Value;
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_token(
        to_check: Result<(CompleteByteSlice, Token), nom::Err<CompleteByteSlice>>,
        against: Token,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_dwells() {
        check_token(dwell(Cbs(b"G04 P10")), Token::Dwell(Value::Float(10.0)));
        check_token(dwell(Cbs(b"G04 P3")), Token::Dwell(Value::Float(3.0)));
        check_token(dwell(Cbs(b"G04 P0.5")), Token::Dwell(Value::Float(0.5)));
        check_token(dwell(Cbs(b"G4 P0.5")), Token::Dwell(Value::Float(0.5)));
        check_token(dwell(Cbs(b"g4p0.5")), Token::Dwell(Value::Float(0.5)));
    }
}
