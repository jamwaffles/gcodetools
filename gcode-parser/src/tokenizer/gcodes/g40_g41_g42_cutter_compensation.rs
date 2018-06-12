use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::value::{preceded_unsigned_value, Value};
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum CutterCompensation {
    Off,
    Left(Option<Value>),
    Right(Option<Value>),
}

named!(pub cutter_compensation<CompleteByteSlice, Token>,
    map!(
        alt!(
            map!(call!(g, 40.0), |_| CutterCompensation::Off) |
            map!(
                ws!(preceded!(call!(g, 41.0), opt!(call!(preceded_unsigned_value, "D")))),
                |tool| CutterCompensation::Left(tool)
            ) |
            map!(
                ws!(preceded!(call!(g, 42.0), opt!(call!(preceded_unsigned_value, "D")))),
                |tool| CutterCompensation::Right(tool)
            )
        ),
        |res| Token::CutterCompensation(res)
    )
);

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
    fn it_parses_cutter_comp() {
        check_token(
            cutter_compensation(Cbs(b"G40")),
            Token::CutterCompensation(CutterCompensation::Off),
        );

        check_token(
            cutter_compensation(Cbs(b"G41 D1")),
            Token::CutterCompensation(CutterCompensation::Left(Some(Value::Unsigned(1u32)))),
        );

        check_token(
            cutter_compensation(Cbs(b"G42 D1")),
            Token::CutterCompensation(CutterCompensation::Right(Some(Value::Unsigned(1u32)))),
        );

        check_token(
            cutter_compensation(Cbs(b"G42 D0")),
            Token::CutterCompensation(CutterCompensation::Right(Some(Value::Unsigned(0u32)))),
        );

        check_token(
            cutter_compensation(Cbs(b"G42")),
            Token::CutterCompensation(CutterCompensation::Right(None)),
        );
    }
}
