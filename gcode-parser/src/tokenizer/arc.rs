use nom::types::CompleteByteSlice;

use super::value::*;
use super::Token;

#[derive(Debug, PartialEq)]
pub struct CenterFormatArc {
    pub x: Option<Value>,
    pub y: Option<Value>,
    pub z: Option<Value>,
    pub i: Option<Value>,
    pub j: Option<Value>,
    pub k: Option<Value>,
    pub p: Option<Value>,
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

named!(pub center_format_arc<CompleteByteSlice, Token>, map_res!(
    do_parse!(
        coords: opt!(ws!(many_m_n!(2, 3, call!(preceded_one_of_float_value, "XYZ")))) >>
        params: ws!(many_m_n!(1, 2, call!(preceded_one_of_float_value, "IJK"))) >>
        p: opt!(call!(preceded_unsigned_value, "P")) >>
        ({
            let mut arc = CenterFormatArc { p, ..Default::default() };

            for (letter, value) in coords.unwrap_or(vec![]).into_iter().chain(params.into_iter()) {
                match letter {
                    'X' => arc.x = Some(value),
                    'Y' => arc.y = Some(value),
                    'Z' => arc.z = Some(value),
                    'I' => arc.i = Some(value),
                    'J' => arc.j = Some(value),
                    'K' => arc.k = Some(value),
                    _ => ()
                }
            };

            arc
        })
    ),
    |res: CenterFormatArc| {
        if res.i.is_none() && res.j.is_none()&& res.k.is_none() {
            Err(())
        } else {
            Ok(Token::CenterFormatArc(res))
        }
    }
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
    fn it_ignores_linear_moves() {
        assert!(center_format_arc(Cbs(b"X0Y0Z0")).is_err());
        assert!(center_format_arc(Cbs(b"Y0Z0")).is_err());
    }

    #[test]
    fn it_handles_no_whitespace() {
        check_token(
            center_format_arc(Cbs(b"X5.0417Y1.9427I-0.3979J0.3028")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(Value::Float(5.0417)),
                y: Some(Value::Float(1.9427)),
                k: None,
                i: Some(Value::Float(-0.3979)),
                j: Some(Value::Float(0.3028)),
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
                x: Some(Value::Float(1.0)),
                y: Some(Value::Float(2.0)),
                z: None,
                i: Some(Value::Float(3.0)),
                j: Some(Value::Float(4.0)),
                k: None,
                p: None,
            }),
        );

        check_token(
            center_format_arc(Cbs(b"X1 Y2 Z5 I3 J4 P6")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(Value::Float(1.0)),
                y: Some(Value::Float(2.0)),
                z: Some(Value::Float(5.0)),
                i: Some(Value::Float(3.0)),
                j: Some(Value::Float(4.0)),
                k: None,
                p: Some(Value::Unsigned(6)),
            }),
        );

        check_token(
            center_format_arc(Cbs(b"X1 Y1 z 20 I20 J0")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(Value::Float(1.0)),
                y: Some(Value::Float(1.0)),
                z: Some(Value::Float(20.0)),
                i: Some(Value::Float(20.0)),
                j: Some(Value::Float(0.0)),
                k: None,
                p: None,
            }),
        );
    }

    #[test]
    fn it_parses_xz_center_format_arcs() {
        check_token(
            center_format_arc(Cbs(b"X1 Z2 I3 K4")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(Value::Float(1.0)),
                y: None,
                z: Some(Value::Float(2.0)),
                i: Some(Value::Float(3.0)),
                j: None,
                k: Some(Value::Float(4.0)),
                p: None,
            }),
        );

        check_token(
            center_format_arc(Cbs(b"X1 Z2 Y5 I3 K4 P6")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(Value::Float(1.0)),
                y: Some(Value::Float(5.0)),
                z: Some(Value::Float(2.0)),
                i: Some(Value::Float(3.0)),
                j: None,
                k: Some(Value::Float(4.0)),
                p: Some(Value::Unsigned(6)),
            }),
        );
    }

    #[test]
    fn it_parses_yz_center_format_arcs() {
        check_token(
            center_format_arc(Cbs(b"Y1 Z2 J3 K4")),
            Token::CenterFormatArc(CenterFormatArc {
                x: None,
                y: Some(Value::Float(1.0)),
                z: Some(Value::Float(2.0)),
                i: None,
                j: Some(Value::Float(3.0)),
                k: Some(Value::Float(4.0)),
                p: None,
            }),
        );

        check_token(
            center_format_arc(Cbs(b"Y1 Z2 X5 J3 K4 P6")),
            Token::CenterFormatArc(CenterFormatArc {
                x: Some(Value::Float(5.0)),
                y: Some(Value::Float(1.0)),
                z: Some(Value::Float(2.0)),
                i: None,
                j: Some(Value::Float(3.0)),
                k: Some(Value::Float(4.0)),
                p: Some(Value::Unsigned(6)),
            }),
        );

        check_token(
            center_format_arc(Cbs(b"Y20.9595 Z-0.5838 I-1.5875 J0.0066")),
            Token::CenterFormatArc(CenterFormatArc {
                x: None,
                y: Some(Value::Float(20.9595)),
                z: Some(Value::Float(-0.5838)),
                i: Some(Value::Float(-1.5875)),
                j: Some(Value::Float(0.0066)),
                k: None,
                p: None,
            }),
        );
    }

    #[test]
    fn it_parses_optional_coords() {
        check_token(
            center_format_arc(Cbs(b"i.5 j.5")),
            Token::CenterFormatArc(CenterFormatArc {
                i: Some(Value::Float(0.5)),
                j: Some(Value::Float(0.5)),
                ..Default::default()
            }),
        );
    }
}
