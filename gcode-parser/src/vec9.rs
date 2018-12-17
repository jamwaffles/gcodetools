//! 9-dimension vector used for all linear moves

use super::value::*;
use nom::space0;
use nom::types::CompleteByteSlice;

use super::Token;

#[derive(Debug, PartialEq)]
pub struct Vec9 {
    pub x: Option<Value>,
    pub y: Option<Value>,
    pub z: Option<Value>,
    pub a: Option<Value>,
    pub b: Option<Value>,
    pub c: Option<Value>,
    pub u: Option<Value>,
    pub v: Option<Value>,
    pub w: Option<Value>,
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
        sep!(space0, do_parse!(
            x: opt!(call!(preceded_float_value, "X")) >>
            y: opt!(call!(preceded_float_value, "Y")) >>
            z: opt!(call!(preceded_float_value, "Z")) >>
            a: opt!(call!(preceded_float_value, "A")) >>
            b: opt!(call!(preceded_float_value, "B")) >>
            c: opt!(call!(preceded_float_value, "C")) >>
            u: opt!(call!(preceded_float_value, "U")) >>
            v: opt!(call!(preceded_float_value, "V")) >>
            w: opt!(call!(preceded_float_value, "W")) >>
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

    #[test]
    fn it_parses_vectors() {
        assert_complete_parse!(
            vec9(Cbs(b"X0 Y1 Z2")),
            Vec9 {
                x: Some(Value::Float(0.0f32)),
                y: Some(Value::Float(1.0f32)),
                z: Some(Value::Float(2.0f32)),
                ..Default::default()
            }
        );

        assert_complete_parse!(
            vec9(Cbs(b"X0Y1Z2")),
            Vec9 {
                x: Some(Value::Float(0.0f32)),
                y: Some(Value::Float(1.0f32)),
                z: Some(Value::Float(2.0f32)),
                ..Default::default()
            }
        );

        assert_complete_parse!(
            vec9(Cbs(b"X-0.5 Y-2 Z100")),
            Vec9 {
                x: Some(Value::Float(-0.5f32)),
                y: Some(Value::Float(-2.0f32)),
                z: Some(Value::Float(100.0f32)),
                ..Default::default()
            }
        );

        assert_complete_parse!(
            vec9(Cbs(b"Z1")),
            Vec9 {
                z: Some(Value::Float(1.0f32)),
                ..Default::default()
            }
        );

        assert_complete_parse!(
            vec9(Cbs(b"U2.5 V3.5 W4.5")),
            Vec9 {
                u: Some(Value::Float(2.5f32)),
                v: Some(Value::Float(3.5f32)),
                w: Some(Value::Float(4.5f32)),
                ..Default::default()
            }
        );

        assert_eq!(
            vec9(Cbs(b"X10 Y20 X30 Y40")),
            Ok((
                Cbs(b"X30 Y40"),
                Vec9 {
                    x: Some(Value::Float(10.0f32)),
                    y: Some(Value::Float(20.0f32)),
                    ..Default::default()
                }
            ))
        );
    }

    #[test]
    fn newlines_make_new_coords() {
        let input = Cbs(b"X3.4000Y8.9431\nZ0.05");

        assert_eq!(
            vec9(input),
            Ok((
                Cbs(b"\nZ0.05"),
                Vec9 {
                    x: Some(Value::Float(3.4f32)),
                    y: Some(Value::Float(8.9431f32)),
                    ..Default::default()
                },
            ))
        );
    }
}
