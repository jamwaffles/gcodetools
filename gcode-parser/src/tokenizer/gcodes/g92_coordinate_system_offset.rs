use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

named!(pub coordinate_system_offset<CompleteByteSlice, Token>, alt!(
    map!(call!(g, 92.0), |_| Token::CoordinateSystemOffset) |
    map!(call!(g, 92.1), |_| Token::CoordinateSystemOffsetHardReset) |
    map!(call!(g, 92.2), |_| Token::CoordinateSystemOffsetSoftReset)
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
    fn it_parses_coord_system_hard_resets() {
        check_token(
            coordinate_system_offset(Cbs(b"G92.1")),
            Token::CoordinateSystemOffsetHardReset,
        );
    }

    #[test]
    fn it_parses_coord_system_soft_resets() {
        check_token(
            coordinate_system_offset(Cbs(b"G92.2")),
            Token::CoordinateSystemOffsetSoftReset,
        );
    }
}
