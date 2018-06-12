use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum PredefinedPosition {
    G28,
    G30,
}

named!(pub predefined_position<CompleteByteSlice, Token>, alt!(
    map!(call!(g, 28.0), |_| Token::GoToPredefinedPosition(PredefinedPosition::G28)) |
    map!(call!(g, 30.0), |_| Token::GoToPredefinedPosition(PredefinedPosition::G30)) |
    map!(call!(g, 28.1), |_| Token::StorePredefinedPosition(PredefinedPosition::G28)) |
    map!(call!(g, 30.1), |_| Token::StorePredefinedPosition(PredefinedPosition::G30))
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
    fn it_goes_to_predefined_position() {
        check_token(
            predefined_position(Cbs(b"G28")),
            Token::GoToPredefinedPosition(PredefinedPosition::G28),
        );
        check_token(
            predefined_position(Cbs(b"G30")),
            Token::GoToPredefinedPosition(PredefinedPosition::G30),
        );
    }

    #[test]
    fn it_stores_predefined_position() {
        check_token(
            predefined_position(Cbs(b"G28.1")),
            Token::StorePredefinedPosition(PredefinedPosition::G28),
        );
        check_token(
            predefined_position(Cbs(b"G30.1")),
            Token::StorePredefinedPosition(PredefinedPosition::G30),
        );
    }
}
