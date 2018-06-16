use nom::types::CompleteByteSlice;

use super::super::{preceded_float_value, Value};
use super::GCode;

#[derive(Debug, PartialEq)]
pub struct ConstantSurfaceSpeed {
    max_spindle_rpm: Option<Value>,
    surface_speed: Value,
}

#[derive(Debug, PartialEq)]
pub enum SpindleControlMode {
    ConstantSurfaceSpeed(ConstantSurfaceSpeed),
    Rpm,
}

named!(pub lathe_spindle_control_mode<CompleteByteSlice, GCode>, map!(
    alt!(
        map!(
            ws!(preceded!(
                g_code!("96"),
                permutation!(
                    call!(preceded_float_value, "D")?,
                    call!(preceded_float_value, "S")
                )
            )),
            |(max_spindle_rpm, surface_speed)| {
                SpindleControlMode::ConstantSurfaceSpeed(ConstantSurfaceSpeed {
                    max_spindle_rpm,
                    surface_speed
                })
            }
        ) |
        g_code!("97", SpindleControlMode::Rpm)
    ),
    |res| GCode::SpindleControlMode(res)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_lathe_spindle_control_modes() {
        assert_complete_parse!(
            lathe_spindle_control_mode(Cbs(b"G97")),
            GCode::SpindleControlMode(SpindleControlMode::Rpm)
        );

        assert_complete_parse!(
            lathe_spindle_control_mode(Cbs(b"G96 S100 D200")),
            GCode::SpindleControlMode(SpindleControlMode::ConstantSurfaceSpeed(
                ConstantSurfaceSpeed {
                    max_spindle_rpm: Some(Value::Float(200.0)),
                    surface_speed: Value::Float(100.0)
                }
            ))
        );

        assert_complete_parse!(
            lathe_spindle_control_mode(Cbs(b"G96 S100")),
            GCode::SpindleControlMode(SpindleControlMode::ConstantSurfaceSpeed(
                ConstantSurfaceSpeed {
                    max_spindle_rpm: None,
                    surface_speed: Value::Float(100.0)
                }
            ))
        );

        assert!(lathe_spindle_control_mode(Cbs(b"G96 D100")).is_err());
        assert!(lathe_spindle_control_mode(Cbs(b"G96")).is_err());
    }
}
