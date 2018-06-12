use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum LatheMeasurementMode {
    Radius,
    Diameter,
}

named!(pub lathe_measurement_mode<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(g, 7.0), |_| LatheMeasurementMode::Diameter) |
        map!(call!(g, 8.0), |_| LatheMeasurementMode::Radius)
    ),
    |res| Token::LatheMeasurementMode(res)
));

#[cfg(test)]
mod tests {
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
    fn it_parses_lathe_measurement_mode() {
        check_token(
            lathe_measurement_mode(Cbs(b"G7")),
            Token::LatheMeasurementMode(LatheMeasurementMode::Diameter),
        );
        check_token(
            lathe_measurement_mode(Cbs(b"G8")),
            Token::LatheMeasurementMode(LatheMeasurementMode::Radius),
        );
    }
}