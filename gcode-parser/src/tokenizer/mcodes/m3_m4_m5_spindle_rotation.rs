use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

/// Spindle rotation
#[derive(Debug, PartialEq)]
pub enum SpindleRotation {
    /// Clockwise (M3)
    Cw,
    /// Counterclockwise (M4)
    Ccw,
    /// Stop (M5)
    Stop,
}

named!(pub spindle_rotation<CompleteByteSlice, Token>, map!(
    alt!(
        map!(call!(m, 3.0), |_| SpindleRotation::Cw) |
        map!(call!(m, 4.0), |_| SpindleRotation::Ccw) |
        map!(call!(m, 5.0), |_| SpindleRotation::Stop)
    ),
    |res| Token::SpindleRotation(res)
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
    fn it_parses_spindle_rotation() {
        check_token(
            spindle_rotation(Cbs(b"M3")),
            Token::SpindleRotation(SpindleRotation::Cw),
        );
        check_token(
            spindle_rotation(Cbs(b"M4")),
            Token::SpindleRotation(SpindleRotation::Ccw),
        );
        check_token(
            spindle_rotation(Cbs(b"M5")),
            Token::SpindleRotation(SpindleRotation::Stop),
        );

        // It gets confused with M30
        assert_eq!(
            spindle_rotation(Cbs(b"M30")),
            Err(nom::Err::Error(nom::simple_errors::Context::Code(
                CompleteByteSlice(b"M30"),
                nom::ErrorKind::Alt
            )))
        );
    }
}
