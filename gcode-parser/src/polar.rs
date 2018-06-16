//! 9-dimension vector used for all linear moves

use super::value::*;
use nom::types::CompleteByteSlice;

use super::Token;

#[derive(Debug, PartialEq)]
pub struct PolarCoordinate {
    pub distance: Option<Value>,
    pub angle: Option<Value>,
}

named!(
    pub polar_coordinate<CompleteByteSlice, Token>,
    map_res!(
        ws!(tuple!(
            opt!(call!(preceded_float_value, "@")),
            opt!(call!(preceded_float_value, "^"))
        )),
        |res| {
            match res {
                (None, None) => Err(()),
                _ => Ok(Token::PolarCoordinate(PolarCoordinate { distance: res.0, angle: res.1 }))
            }
        }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_polar_coordinates() {
        assert_complete_parse!(
            polar_coordinate(Cbs(b"@.5 ^90")),
            Token::PolarCoordinate(PolarCoordinate {
                distance: Some(Value::Float(0.5)),
                angle: Some(Value::Float(90.0)),
            })
        );

        assert_complete_parse!(
            polar_coordinate(Cbs(b"^90")),
            Token::PolarCoordinate(PolarCoordinate {
                distance: None,
                angle: Some(Value::Float(90.0)),
            })
        );

        assert_complete_parse!(
            polar_coordinate(Cbs(b"@.5")),
            Token::PolarCoordinate(PolarCoordinate {
                distance: Some(Value::Float(0.5)),
                angle: None,
            })
        );
    }
}
