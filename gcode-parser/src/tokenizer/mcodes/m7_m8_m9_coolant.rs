use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum Coolant {
    Mist,
    Flood,
    Off,
}

named!(pub coolant<CompleteByteSlice, Token>, alt!(
    map!(call!(m, 7.0), |_| Token::Coolant(Coolant::Mist)) |
    map!(call!(m, 8.0), |_| Token::Coolant(Coolant::Flood)) |
    map!(call!(m, 9.0), |_| Token::Coolant(Coolant::Off))
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
