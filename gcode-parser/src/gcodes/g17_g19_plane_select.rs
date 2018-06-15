use nom::types::CompleteByteSlice;

use super::GCode;

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

named!(pub plane_select<CompleteByteSlice, GCode>, map!(
    alt!(
        g_code!("17.1", Plane::Uv) |
        g_code!("18.1", Plane::Wu) |
        g_code!("19.1", Plane::Vw) |
        g_code!("17", Plane::Xy) |
        g_code!("18", Plane::Zx) |
        g_code!("19", Plane::Yz)
    ),
    |res| GCode::PlaneSelect(res)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_token(
        to_check: Result<(CompleteByteSlice, GCode), nom::Err<CompleteByteSlice>>,
        against: GCode,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_plane_select() {
        check_token(plane_select(Cbs(b"G17")), GCode::PlaneSelect(Plane::Xy));
        check_token(plane_select(Cbs(b"G18")), GCode::PlaneSelect(Plane::Zx));
        check_token(plane_select(Cbs(b"G19")), GCode::PlaneSelect(Plane::Yz));
        check_token(plane_select(Cbs(b"G17.1")), GCode::PlaneSelect(Plane::Uv));
        check_token(plane_select(Cbs(b"G18.1")), GCode::PlaneSelect(Plane::Wu));
        check_token(plane_select(Cbs(b"G19.1")), GCode::PlaneSelect(Plane::Vw));
    }
}
