use nom::types::CompleteByteSlice;

use super::Token;
use super::helpers::*;

#[derive(Debug, PartialEq)]
pub struct CenterFormatArc {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub i: Option<f32>,
    pub j: Option<f32>,
    pub k: Option<f32>,
    pub p: Option<u32>,
}

impl Default for CenterFormatArc {
    fn default() -> CenterFormatArc {
        CenterFormatArc {
            x: None,
            y: None,
            z: None,
            i: None,
            j: None,
            k: None,
            p: None,
        }
    }
}

named!(xy_plane<CompleteByteSlice, CenterFormatArc>,
    ws!(do_parse!(
        x: call!(preceded_f32, "X") >>
        y: call!(preceded_f32, "Y") >>
        z: opt!(call!(preceded_f32, "Z")) >>
        i: call!(preceded_f32, "I") >>
        j: call!(preceded_f32, "J") >>
        p: opt!(call!(preceded_u32, "P")) >>
        ({
            CenterFormatArc {
                x: Some(x), y: Some(y), z, i: Some(i), j: Some(j), k: None, p
            }
        })
    ))
);

named!(xz_plane<CompleteByteSlice, CenterFormatArc>,
    ws!(do_parse!(
        x: call!(preceded_f32, "X") >>
        z: call!(preceded_f32, "Z") >>
        y: opt!(call!(preceded_f32, "Y")) >>
        i: call!(preceded_f32, "I") >>
        k: call!(preceded_f32, "K") >>
        p: opt!(call!(preceded_u32, "P")) >>
        ({
            CenterFormatArc {
                x: Some(x), y, z: Some(z), i: Some(i), j: None, k: Some(k), p
            }
        })
    ))
);

named!(yz_plane<CompleteByteSlice, CenterFormatArc>,
    ws!(do_parse!(
        y: call!(preceded_f32, "Y") >>
        z: call!(preceded_f32, "Z") >>
        x: opt!(call!(preceded_f32, "X")) >>
        j: call!(preceded_f32, "J") >>
        k: call!(preceded_f32, "K") >>
        p: opt!(call!(preceded_u32, "P")) >>
        ({
            CenterFormatArc {
                x, y: Some(y), z: Some(z), i: None, j: Some(j), k: Some(k), p
            }
        })
    ))
);

named!(pub center_format_arc<CompleteByteSlice, Token>, map!(
    alt_complete!(xy_plane | xz_plane | yz_plane),
    |res| Token::CenterFormatArc(res)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    fn check_token(
        to_check: Result<(CompleteByteSlice, Token), nom::Err<CompleteByteSlice>>,
        against: Token,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    const EMPTY: Cbs = Cbs(b"");

    #[test]
    fn it_handles_no_whitespace() {
        check_token(
            center_format_arc(Cbs(b"X5.0417Y1.9427I-0.3979J0.3028")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(5.0417),
                y: Some(1.9427),
                k: None,
                i: Some(-0.3979),
                j: Some(0.3028),
                z: None,
                p: None,
            }),
        );
    }

    #[test]
    fn it_parses_xy_center_format_arcs() {
        check_token(
            center_format_arc(Cbs(b"X1 Y2 I3 J4")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(1.0),
                y: Some(2.0),
                z: None,
                i: Some(3.0),
                j: Some(4.0),
                k: None,
                p: None,
            }),
        );

        check_token(
            center_format_arc(Cbs(b"X1 Y2 Z5 I3 J4 P6")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(1.0),
                y: Some(2.0),
                z: Some(5.0),
                i: Some(3.0),
                j: Some(4.0),
                k: None,
                p: Some(6),
            }),
        );
    }

    #[test]
    fn it_parses_xz_center_format_arcs() {
        check_token(
            center_format_arc(Cbs(b"X1 Z2 I3 K4")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(1.0),
                y: None,
                z: Some(2.0),
                i: Some(3.0),
                j: None,
                k: Some(4.0),
                p: None,
            }),
        );

        check_token(
            center_format_arc(Cbs(b"X1 Z2 Y5 I3 K4 P6")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(1.0),
                y: Some(5.0),
                z: Some(2.0),
                i: Some(3.0),
                j: None,
                k: Some(4.0),
                p: Some(6),
            }),
        );
    }

    #[test]
    fn it_parses_yz_center_format_arcs() {
        check_token(
            center_format_arc(Cbs(b"Y1 Z2 J3 K4")),
            Token::CenterFormatArc(CenterFormatArc {
                x: None,
                y: Some(1.0),
                z: Some(2.0),
                i: None,
                j: Some(3.0),
                k: Some(4.0),
                p: None,
            }),
        );

        check_token(
            center_format_arc(Cbs(b"Y1 Z2 X5 J3 K4 P6")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(5.0),
                y: Some(1.0),
                z: Some(2.0),
                i: None,
                j: Some(3.0),
                k: Some(4.0),
                p: Some(6),
            }),
        );
    }
}
