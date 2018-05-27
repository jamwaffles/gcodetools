//! 9-dimension vector used for all linear moves

use nom::types::CompleteByteSlice;

use super::helpers::*;
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
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

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
}
