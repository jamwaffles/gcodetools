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
        map!(call!(g, 17.1), |_| Plane::Uv) |
        map!(call!(g, 18.1), |_| Plane::Wu) |
        map!(call!(g, 19.1), |_| Plane::Vw) |
        map!(call!(g, 17.0), |_| Plane::Xy) |
        map!(call!(g, 18.0), |_| Plane::Zx) |
        map!(call!(g, 19.0), |_| Plane::Yz)
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
