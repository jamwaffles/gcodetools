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
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_units() {
        assert_complete_parse!(units(Cbs(b"G20")), GCode::Units(Units::Inch));
        assert_complete_parse!(units(Cbs(b"G21")), GCode::Units(Units::Mm));
    }
}
