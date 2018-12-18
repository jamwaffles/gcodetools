use nom::types::CompleteByteSlice;

use super::super::vec9::{vec9, Vec9};
use super::GCode;

/// Define which probing routine to use
#[derive(Debug, PartialEq)]
pub enum StraightProbe {
    /// Probe towards, stop on contact
    Towards(Vec9),
    /// Probe towards, stop on contact, error if no contact at move end
    TowardsWithError(Vec9),
    /// Probe away, stop on contact loss
    Away(Vec9),
    /// Probe away, stop on contact loss, error if no contact loss at move end
    AwayWithError(Vec9),
}

named!(pub straight_probe<CompleteByteSlice, GCode>, map!(
    alt!(
        map!(ws!(preceded!(g_code!("38.2"), vec9)), |pos| StraightProbe::Towards(pos)) |
        map!(ws!(preceded!(g_code!("38.3"), vec9)), |pos| StraightProbe::TowardsWithError(pos)) |
        map!(ws!(preceded!(g_code!("38.4"), vec9)), |pos| StraightProbe::Away(pos)) |
        map!(ws!(preceded!(g_code!("38.5"), vec9)), |pos| StraightProbe::AwayWithError(pos))
    ),
    |res| GCode::StraightProbe(res)
));

#[cfg(test)]
mod tests {
    use super::super::super::value::Value;
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_straight_probes() {
        let cases: Vec<(&str, GCode)> = vec![
            (
                "G38.2 X10",
                GCode::StraightProbe(StraightProbe::Towards(Vec9 {
                    x: Some(Value::Float(10.0)),
                    ..Default::default()
                })),
            ),
            (
                "G38.3 Y10 Z10",
                GCode::StraightProbe(StraightProbe::TowardsWithError(Vec9 {
                    y: Some(Value::Float(10.0)),
                    z: Some(Value::Float(10.0)),
                    ..Default::default()
                })),
            ),
            (
                "G38.4 X10",
                GCode::StraightProbe(StraightProbe::Away(Vec9 {
                    x: Some(Value::Float(10.0)),
                    ..Default::default()
                })),
            ),
            (
                "G38.5 Y10 Z10",
                GCode::StraightProbe(StraightProbe::AwayWithError(Vec9 {
                    y: Some(Value::Float(10.0)),
                    z: Some(Value::Float(10.0)),
                    ..Default::default()
                })),
            ),
        ];

        for (test, expected) in cases {
            assert_complete_parse!(straight_probe(Cbs(test.as_bytes())), expected);
        }
    }
}
