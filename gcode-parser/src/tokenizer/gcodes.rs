use nom::types::CompleteByteSlice;

use super::Token;

use super::helpers::*;

#[derive(Debug, PartialEq)]
pub enum Units {
    Inch,
    Mm,
}

#[derive(Debug, PartialEq)]
pub enum DistanceMode {
    Absolute,
    Incremental,
}

#[derive(Debug, PartialEq)]
pub struct PathBlending {
    pub p: Option<f32>,
    pub q: Option<f32>,
}

#[derive(Debug, PartialEq)]
pub enum CutterCompensation {
    Off,
}

#[derive(Debug, PartialEq)]
pub enum Plane {
    Xy,
    Zx,
    Yz,
    Uv,
    Wu,
    Vw,
}

named!(pub units<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("G20"), |_| Units::Inch) |
        map!(tag_no_case!("G21"), |_| Units::Mm)
    ),
    |res| Token::Units(res)
));

named!(pub distance_mode<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("G90"), |_| DistanceMode::Absolute) |
        map!(tag_no_case!("G91"), |_| DistanceMode::Incremental)
    ),
    |res| Token::DistanceMode(res)
));

named!(pub path_blending<CompleteByteSlice, Token>, ws!(
    do_parse!(
        tag_no_case!("G64") >>
        p: opt!(call!(preceded_f32, "P")) >>
        q: opt!(call!(preceded_f32, "Q")) >> ({
            Token::PathBlending(PathBlending { p, q })
        })
    )
));

named!(pub cutter_compensation<CompleteByteSlice, Token>,
    map!(
        alt!(
            map!(tag_no_case!("G40"), |_| CutterCompensation::Off)
        ),
        |res| Token::CutterCompensation(res)
    )
);

named!(pub rapid_move<CompleteByteSlice, Token>,
    map!(ws!(preceded!(tag!("G0"), vec9)), |res| Token::RapidMove(res))
);

named!(pub linear_move<CompleteByteSlice, Token>,
    map!(ws!(preceded!(tag!("G1"), vec9)), |res| Token::LinearMove(res))
);

named!(pub plane_select<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("G17.1"), |_| Plane::Uv) |
        map!(tag_no_case!("G18.1"), |_| Plane::Wu) |
        map!(tag_no_case!("G19.1"), |_| Plane::Vw) |
        map!(tag_no_case!("G17"), |_| Plane::Xy) |
        map!(tag_no_case!("G18"), |_| Plane::Zx) |
        map!(tag_no_case!("G19"), |_| Plane::Yz)
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

    #[test]
    fn it_parses_rapids() {
        check_token(
            rapid_move(Cbs(b"G0 X0 Y1 Z2")),
            Token::RapidMove(Vec9 {
                x: Some(0.0f32),
                y: Some(1.0f32),
                z: Some(2.0f32),
                ..Default::default()
            }),
        );
    }

    #[test]
    fn it_parses_linear_moves() {
        check_token(
            linear_move(Cbs(b"G1 X0 Y1 Z2")),
            Token::LinearMove(Vec9 {
                x: Some(0.0f32),
                y: Some(1.0f32),
                z: Some(2.0f32),
                ..Default::default()
            }),
        );
    }

    #[test]
    fn it_parses_cutter_comp() {
        check_token(
            cutter_compensation(Cbs(b"G40")),
            Token::CutterCompensation(CutterCompensation::Off),
        );

        // TODO
        // assert_eq!(
        //     path_blending(Cbs(b"G64 Q0.02")),
        //     Ok((
        //         EMPTY,
        //         Token::PathBlending(PathBlending { p: None, q: None })
        //     ))
        // );
    }

    #[test]
    fn it_parses_blending_mode() {
        check_token(
            path_blending(Cbs(b"G64")),
            Token::PathBlending(PathBlending { p: None, q: None }),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01")),
            Token::PathBlending(PathBlending {
                p: Some(0.01f32),
                q: None,
            }),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01 Q0.02")),
            Token::PathBlending(PathBlending {
                p: Some(0.01f32),
                q: Some(0.02f32),
            }),
        );

        // TODO
        // check_token(
        //     path_blending(Cbs(b"G64 Q0.02")),
        //     Token::PathBlending(PathBlending { p: None, q: None })
        // );
    }

    #[test]
    fn it_parses_distance_mode() {
        check_token(
            distance_mode(Cbs(b"G90")),
            Token::DistanceMode(DistanceMode::Absolute),
        );

        check_token(
            distance_mode(Cbs(b"G91")),
            Token::DistanceMode(DistanceMode::Incremental),
        );
    }
    #[test]
    fn it_parses_units() {
        check_token(units(Cbs(b"G20")), Token::Units(Units::Inch));
        check_token(units(Cbs(b"G21")), Token::Units(Units::Mm));
    }
}
