use nom::types::CompleteByteSlice;

use super::GCode;

/// Units selection
#[derive(Debug, PartialEq)]
pub enum Units {
    /// Use inch units for all distances
    Inch,
    /// Use millimeter units for all distances
    Mm,
}

named!(pub units<CompleteByteSlice, GCode>, map!(
    alt!(
        g_code!("20", Units::Inch) |
        g_code!("21", Units::Mm)
    ),
    |res| GCode::Units(res)
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
    fn it_parses_units() {
        check_token(units(Cbs(b"G20")), GCode::Units(Units::Inch));
        check_token(units(Cbs(b"G21")), GCode::Units(Units::Mm));
    }
}
