use nom::types::CompleteByteSlice;

use super::super::value::{preceded_float_value, Value};
use super::GCode;

#[derive(Debug, PartialEq)]
pub enum PathBlendingMode {
    Blended((Option<Value>, Option<Value>)),
    ExactPath,
    // TODO
    // ExactStop,
}

named!(pub path_blending<CompleteByteSlice, GCode>, map!(
    alt!(
        ws!(do_parse!(
            g_code!("64") >>
            p: opt!(call!(preceded_float_value, "P")) >>
            q: opt!(call!(preceded_float_value, "Q")) >> ({
                PathBlendingMode::Blended((p, q))
            })
        )) |
        g_code!("61", PathBlendingMode::ExactPath)
    ),
    |res| GCode::PathBlendingMode(res)
));

#[cfg(test)]
mod tests {
    use super::super::super::value::Value;
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_blending_mode() {
        assert_complete_parse!(
            path_blending(Cbs(b"G64")),
            GCode::PathBlendingMode(PathBlendingMode::Blended((None, None)))
        );

        assert_complete_parse!(
            path_blending(Cbs(b"G64 P0.01")),
            GCode::PathBlendingMode(PathBlendingMode::Blended((
                Some(Value::Float(0.01f32)),
                None,
            )))
        );

        assert_complete_parse!(
            path_blending(Cbs(b"G64 P0.01 Q0.02")),
            GCode::PathBlendingMode(PathBlendingMode::Blended((
                Some(Value::Float(0.01f32)),
                Some(Value::Float(0.02f32)),
            )))
        );

        // TODO
        // assert_complete_parse!(
        //     path_blending(Cbs(b"G64 Q0.02")),
        //     GCode::PathBlendingMode(PathBlendingMode { p: None, q: None })
        // );
    }
}
