use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::vec9::{vec9, Vec9};
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum StraightProbe {
    Towards(Vec9),
    TowardsWithError(Vec9),
    Away(Vec9),
    AwayWithError(Vec9),
}

named!(pub straight_probe<CompleteByteSlice, Token>, map!(
    alt!(
        map!(ws!(preceded!(call!(g, 38.2), vec9)), |pos| StraightProbe::Towards(pos)) |
        map!(ws!(preceded!(call!(g, 38.3), vec9)), |pos| StraightProbe::TowardsWithError(pos)) |
        map!(ws!(preceded!(call!(g, 38.4), vec9)), |pos| StraightProbe::Away(pos)) |
        map!(ws!(preceded!(call!(g, 38.5), vec9)), |pos| StraightProbe::AwayWithError(pos))
    ),
    |res| Token::StraightProbe(res)
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
    fn it_parses_straight_probes() {
        let cases: Vec<(&str, Token)> = vec![
            (
                "G38.2 X10",
                Token::StraightProbe(StraightProbe::Towards(Vec9 {
                    x: Some(Value::Float(10.0)),
                    ..Default::default()
                })),
            ),
            (
                "G38.3 Y10 Z10",
                Token::StraightProbe(StraightProbe::TowardsWithError(Vec9 {
                    y: Some(Value::Float(10.0)),
                    z: Some(Value::Float(10.0)),
                    ..Default::default()
                })),
            ),
            (
                "G38.4 X10",
                Token::StraightProbe(StraightProbe::Away(Vec9 {
                    x: Some(Value::Float(10.0)),
                    ..Default::default()
                })),
            ),
            (
                "G38.5 Y10 Z10",
                Token::StraightProbe(StraightProbe::AwayWithError(Vec9 {
                    y: Some(Value::Float(10.0)),
                    z: Some(Value::Float(10.0)),
                    ..Default::default()
                })),
            ),
        ];

        for (test, expected) in cases {
            check_token(straight_probe(Cbs(test.as_bytes())), expected);
        }
    }
}
