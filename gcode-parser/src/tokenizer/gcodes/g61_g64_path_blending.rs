use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::value::{preceded_float_value, Value};
use super::super::Token;

#[derive(Debug, PartialEq)]
pub enum PathBlendingMode {
    Blended((Option<Value>, Option<Value>)),
    ExactPath,
    // TODO
    // ExactStop,
}

named!(pub path_blending<CompleteByteSlice, Token>, map!(
    alt!(
        ws!(do_parse!(
            call!(g, 64.0) >>
            p: opt!(call!(preceded_float_value, "P")) >>
            q: opt!(call!(preceded_float_value, "Q")) >> ({
                PathBlendingMode::Blended((p, q))
            })
        )) |
        map!(call!(g, 61.0), |_| PathBlendingMode::ExactPath)
    ),
    |res| Token::PathBlendingMode(res)
));

#[cfg(test)]
mod tests {
    use super::super::super::value::Value;
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
    fn it_parses_blending_mode() {
        check_token(
            path_blending(Cbs(b"G64")),
            Token::PathBlendingMode(PathBlendingMode::Blended((None, None))),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01")),
            Token::PathBlendingMode(PathBlendingMode::Blended((
                Some(Value::Float(0.01f32)),
                None,
            ))),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01 Q0.02")),
            Token::PathBlendingMode(PathBlendingMode::Blended((
                Some(Value::Float(0.01f32)),
                Some(Value::Float(0.02f32)),
            ))),
        );

        // TODO
        // check_token(
        //     path_blending(Cbs(b"G64 Q0.02")),
        //     Token::PathBlendingMode(PathBlendingMode { p: None, q: None })
        // );
    }
}
