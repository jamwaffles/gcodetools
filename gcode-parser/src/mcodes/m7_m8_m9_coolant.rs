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
    m_int!(7, MCode::Coolant(Coolant::Mist)) |
    m_int!(8, MCode::Coolant(Coolant::Flood)) |
    m_int!(9, MCode::Coolant(Coolant::Off))
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
    fn it_parses_coolant() {
        check_token(coolant(Cbs(b"M7")), MCode::Coolant(Coolant::Mist));
        check_token(coolant(Cbs(b"M8")), MCode::Coolant(Coolant::Flood));
        check_token(coolant(Cbs(b"M9")), MCode::Coolant(Coolant::Off));
    }
}
