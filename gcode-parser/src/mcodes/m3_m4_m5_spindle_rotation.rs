use nom::types::CompleteByteSlice;

use super::MCode;

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

named!(pub spindle_rotation<CompleteByteSlice, MCode>, map!(
    alt!(
        m_code!("3", SpindleRotation::Cw) |
        m_code!("4", SpindleRotation::Ccw) |
        m_code!("5", SpindleRotation::Stop)
    ),
    |res| MCode::SpindleRotation(res)
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
    fn it_parses_spindle_rotation() {
        check_token(
            spindle_rotation(Cbs(b"M3")),
            MCode::SpindleRotation(SpindleRotation::Cw),
        );
        check_token(
            spindle_rotation(Cbs(b"M4")),
            MCode::SpindleRotation(SpindleRotation::Ccw),
        );
        check_token(
            spindle_rotation(Cbs(b"M5")),
            MCode::SpindleRotation(SpindleRotation::Stop),
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
