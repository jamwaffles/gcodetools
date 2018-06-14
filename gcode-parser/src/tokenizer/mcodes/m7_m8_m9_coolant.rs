use nom::types::CompleteByteSlice;

use super::super::Token;

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

named!(pub coolant<CompleteByteSlice, Token>, alt!(
    m_int!(7, Token::Coolant(Coolant::Mist)) |
    m_int!(8, Token::Coolant(Coolant::Flood)) |
    m_int!(9, Token::Coolant(Coolant::Off))
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
    fn it_parses_coolant() {
        check_token(coolant(Cbs(b"M7")), Token::Coolant(Coolant::Mist));
        check_token(coolant(Cbs(b"M8")), Token::Coolant(Coolant::Flood));
        check_token(coolant(Cbs(b"M9")), Token::Coolant(Coolant::Off));
    }
}
