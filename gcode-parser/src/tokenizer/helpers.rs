use nom::types::CompleteByteSlice;
use nom::*;

use super::Token;

#[derive(Debug, PartialEq)]
pub struct Vec9 {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub a: Option<f32>,
    pub b: Option<f32>,
    pub c: Option<f32>,
    pub u: Option<f32>,
    pub v: Option<f32>,
    pub w: Option<f32>,
}

impl Default for Vec9 {
    fn default() -> Vec9 {
        Vec9 {
            x: None,
            y: None,
            z: None,
            a: None,
            b: None,
            c: None,
            u: None,
            v: None,
            w: None,
        }
    }
}

named!(pub comment<CompleteByteSlice, Token>, map!(
    flat_map!(
        delimited!(tag!("("), take_until!(")"), tag!(")")),
        parse_to!(String)
    ),
    |res| Token::Comment(res.trim().into())
));

named_args!(
    pub preceded_f32<'a>(preceding: &str)<CompleteByteSlice<'a>, f32>,
    flat_map!(preceded!(tag_no_case!(preceding), recognize!(recognize_float)), parse_to!(f32))
);

named_args!(
    pub preceded_i32<'a>(preceding: &str)<CompleteByteSlice<'a>, i32>,
    flat_map!(preceded!(tag_no_case!(preceding), recognize!(preceded!(opt!(one_of!("+-")), digit))), parse_to!(i32))
);

named_args!(
    pub preceded_u32<'a>(preceding: &str)<CompleteByteSlice<'a>, u32>,
    flat_map!(preceded!(tag_no_case!(preceding), recognize!(digit)), parse_to!(u32))
);

named!(
    pub vec9<CompleteByteSlice, Vec9>,
    map_res!(
        ws!(do_parse!(
            x: opt!(call!(preceded_f32, "X")) >>
            y: opt!(call!(preceded_f32, "Y")) >>
            z: opt!(call!(preceded_f32, "Z")) >>
            a: opt!(call!(preceded_f32, "A")) >>
            b: opt!(call!(preceded_f32, "B")) >>
            c: opt!(call!(preceded_f32, "C")) >>
            u: opt!(call!(preceded_f32, "U")) >>
            v: opt!(call!(preceded_f32, "V")) >>
            w: opt!(call!(preceded_f32, "W")) >>
            (
                Vec9 {
                    x,
                    y,
                    z,
                    a,
                    b,
                    c,
                    u,
                    v,
                    w,
                }
            )
        )),
        |vec| {
            let empty = Vec9 { ..Default::default() };

            if vec == empty {
                Err(())
            } else {
                Ok(vec)
            }
        }
    )
);

named!(
    pub coord<CompleteByteSlice, Token>,
    map!(vec9, |res| Token::Coord(res))
);

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
    fn it_parses_comments() {
        check_token(
            comment(Cbs(b"(Hello world)")),
            Token::Comment("Hello world".into()),
        );

        check_token(
            comment(Cbs(b"( Hello world )")),
            Token::Comment("Hello world".into()),
        );
    }

    #[test]
    fn it_parses_vectors() {
        assert_eq!(
            vec9(Cbs(b"X0 Y1 Z2")),
            Ok((
                EMPTY,
                Vec9 {
                    x: Some(0.0f32),
                    y: Some(1.0f32),
                    z: Some(2.0f32),
                    ..Default::default()
                }
            ))
        );

        assert_eq!(
            vec9(Cbs(b"X0Y1Z2")),
            Ok((
                EMPTY,
                Vec9 {
                    x: Some(0.0f32),
                    y: Some(1.0f32),
                    z: Some(2.0f32),
                    ..Default::default()
                }
            ))
        );

        assert_eq!(
            vec9(Cbs(b"X-0.5 Y-2 Z100")),
            Ok((
                EMPTY,
                Vec9 {
                    x: Some(-0.5f32),
                    y: Some(-2.0f32),
                    z: Some(100.0f32),
                    ..Default::default()
                }
            ))
        );

        assert_eq!(
            vec9(Cbs(b"Z1")),
            Ok((
                EMPTY,
                Vec9 {
                    z: Some(1.0f32),
                    ..Default::default()
                }
            ))
        );

        assert_eq!(
            vec9(Cbs(b"U2.5 V3.5 W4.5")),
            Ok((
                EMPTY,
                Vec9 {
                    u: Some(2.5f32),
                    v: Some(3.5f32),
                    w: Some(4.5f32),
                    ..Default::default()
                }
            ))
        );

        assert_eq!(
            vec9(Cbs(b"X10 Y20 X30 Y40")),
            Ok((
                Cbs(b"X30 Y40"),
                Vec9 {
                    x: Some(10.0f32),
                    y: Some(20.0f32),
                    ..Default::default()
                }
            ))
        );
    }

    #[test]
    fn it_parses_preceded_floats() {
        assert_eq!(preceded_f32(Cbs(b"x1."), "X"), Ok((EMPTY, 1.0f32)));

        assert_eq!(preceded_f32(Cbs(b"x1.23"), "X"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"y-1.23"), "Y"), Ok((EMPTY, -1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"z+1.23"), "Z"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"a123"), "A"), Ok((EMPTY, 123.0f32)));

        assert_eq!(preceded_f32(Cbs(b"X1.23"), "X"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"Y-1.23"), "Y"), Ok((EMPTY, -1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"Z+1.23"), "Z"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"A123"), "A"), Ok((EMPTY, 123.0f32)));
    }

    #[test]
    fn it_parses_preceded_signed_integers() {
        assert_eq!(preceded_i32(Cbs(b"x123"), "X"), Ok((EMPTY, 123i32)));
        assert_eq!(preceded_i32(Cbs(b"y-123"), "Y"), Ok((EMPTY, -123i32)));

        assert_eq!(preceded_i32(Cbs(b"X123"), "X"), Ok((EMPTY, 123i32)));
        assert_eq!(preceded_i32(Cbs(b"Y-123"), "Y"), Ok((EMPTY, -123i32)));
    }

    #[test]
    fn it_parses_preceded_unsigned_integers() {
        assert_eq!(preceded_u32(Cbs(b"x123"), "X"), Ok((EMPTY, 123u32)));
        assert_eq!(preceded_u32(Cbs(b"X123"), "X"), Ok((EMPTY, 123u32)));

        assert!(preceded_u32(Cbs(b"y-123"), "Y").is_err());
        assert!(preceded_u32(Cbs(b"Y-123"), "Y").is_err());
    }
}
