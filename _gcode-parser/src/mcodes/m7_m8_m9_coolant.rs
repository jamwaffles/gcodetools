use nom::types::CompleteByteSlice;

use super::MCode;

/// Coolant
#[derive(Debug, PartialEq)]
pub enum Coolant {
    /// Enable mist coolant (M7)
    Mist,
    /// Enable flood coolant (M8)
    Flood,
    /// Disable all coolant (M9)
    Off,
}

named!(pub coolant<CompleteByteSlice, MCode>, alt!(
    m_code!("7", MCode::Coolant(Coolant::Mist)) |
    m_code!("8", MCode::Coolant(Coolant::Flood)) |
    m_code!("9", MCode::Coolant(Coolant::Off))
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_coolant() {
        assert_complete_parse!(coolant(Cbs(b"M7")), MCode::Coolant(Coolant::Mist));
        assert_complete_parse!(coolant(Cbs(b"M8")), MCode::Coolant(Coolant::Flood));
        assert_complete_parse!(coolant(Cbs(b"M9")), MCode::Coolant(Coolant::Off));
    }
}
