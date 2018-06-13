use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

/// Plane select
#[derive(Debug, PartialEq)]
pub enum Plane {
    /// X-Y plane
    Xy,
    /// Z-X plane
    Zx,
    /// Y-Z plane
    Yz,
    /// U-V plane
    Uv,
    /// W-U plane
    Wu,
    /// V-W plane
    Vw,
}

named!(pub plane_select<CompleteByteSlice, Token>, map!(
    alt!(
        g_float!(17.1, Plane::Uv) |
        g_float!(18.1, Plane::Wu) |
        g_float!(19.1, Plane::Vw) |
        g_float!(17.0, Plane::Xy) |
        g_float!(18.0, Plane::Zx) |
        g_float!(19.0, Plane::Yz)
    ),
    |res| Token::PlaneSelect(res)
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
    fn it_parses_plane_select() {
        check_token(plane_select(Cbs(b"G17")), Token::PlaneSelect(Plane::Xy));
        check_token(plane_select(Cbs(b"G18")), Token::PlaneSelect(Plane::Zx));
        check_token(plane_select(Cbs(b"G19")), Token::PlaneSelect(Plane::Yz));
        check_token(plane_select(Cbs(b"G17.1")), Token::PlaneSelect(Plane::Uv));
        check_token(plane_select(Cbs(b"G18.1")), Token::PlaneSelect(Plane::Wu));
        check_token(plane_select(Cbs(b"G19.1")), Token::PlaneSelect(Plane::Vw));
    }
}
