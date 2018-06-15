use nom::types::CompleteByteSlice;

use super::super::value::{float_value, Value};
use super::GCode;

#[derive(Debug, PartialEq)]
pub struct SpindleSyncMotion {
    x: Option<Value>,
    y: Option<Value>,
    z: Option<Value>,
    k: Value,
}

type SyncMotionReturn = (Option<Value>, Option<Value>, Option<Value>, Value);

named!(pub spindle_sync_motion<CompleteByteSlice, GCode>, map_res!(
    ws!(preceded!(
        g_code!("33"),
        permutation!(
            ws!(preceded!(one_of!("Xx"), float_value))?,
            ws!(preceded!(one_of!("Yy"), float_value))?,
            ws!(preceded!(one_of!("Zz"), float_value))?,
            ws!(preceded!(one_of!("Kk"), float_value))
        )
    )),
    |(x, y, z, k): SyncMotionReturn| {
        if x.is_none() && y.is_none() && z.is_none() {
            Err(())
        } else {
            Ok(GCode::SpindleSyncMotion(SpindleSyncMotion {
                x, y, z, k
            }))
        }
    }
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
    fn it_parses_spindle_sync_motion() {
        check_token(
            spindle_sync_motion(Cbs(b"G33 Z-1 K.0625")),
            GCode::SpindleSyncMotion(SpindleSyncMotion {
                x: None,
                y: None,
                z: Some(Value::Float(-1.0)),
                k: Value::Float(0.0625),
            }),
        );
        check_token(
            spindle_sync_motion(Cbs(b"G33 Z-15 K1.5")),
            GCode::SpindleSyncMotion(SpindleSyncMotion {
                x: None,
                y: None,
                z: Some(Value::Float(-15.0)),
                k: Value::Float(1.5),
            }),
        );
        check_token(
            spindle_sync_motion(Cbs(b"G33 Z-2 K0.125")),
            GCode::SpindleSyncMotion(SpindleSyncMotion {
                x: None,
                y: None,
                z: Some(Value::Float(-2.0)),
                k: Value::Float(0.125),
            }),
        );
        check_token(
            spindle_sync_motion(Cbs(b"G33 X10 Z-2 K0.125")),
            GCode::SpindleSyncMotion(SpindleSyncMotion {
                x: Some(Value::Float(10.0)),
                y: None,
                z: Some(Value::Float(-2.0)),
                k: Value::Float(0.125),
            }),
        );
        assert!(spindle_sync_motion(Cbs(b"G33 K10")).is_err());
    }
}
