use nom::types::CompleteByteSlice;

use super::GCode;

#[derive(Debug, PartialEq)]
pub enum LatheMeasurementMode {
    Radius,
    Diameter,
}

named!(pub lathe_measurement_mode<CompleteByteSlice, GCode>, map!(
    alt!(
        g_code!("7", LatheMeasurementMode::Diameter) |
        g_code!("8", LatheMeasurementMode::Radius)
    ),
    |res| GCode::LatheMeasurementMode(res)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_lathe_measurement_mode() {
        assert_complete_parse!(
            lathe_measurement_mode(Cbs(b"G7")),
            GCode::LatheMeasurementMode(LatheMeasurementMode::Diameter)
        );
        assert_complete_parse!(
            lathe_measurement_mode(Cbs(b"G8")),
            GCode::LatheMeasurementMode(LatheMeasurementMode::Radius)
        );
    }
}
