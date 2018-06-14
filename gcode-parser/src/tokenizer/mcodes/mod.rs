mod m0_m1_pause;
mod m100_m199_user_command;
mod m2_m30_end_program;
mod m3_m4_m5_spindle_rotation;
mod m6_tool_change;
mod m70_m71_m72_m73_modal_state;
mod m7_m8_m9_coolant;

use nom::types::CompleteByteSlice;

use self::m0_m1_pause::pause;
use self::m100_m199_user_command::user_command;
use self::m2_m30_end_program::end_program;
use self::m3_m4_m5_spindle_rotation::spindle_rotation;
use self::m6_tool_change::tool_change;
use self::m70_m71_m72_m73_modal_state::modal_state;
use self::m7_m8_m9_coolant::coolant;

pub use self::m3_m4_m5_spindle_rotation::SpindleRotation;
pub use self::m7_m8_m9_coolant::Coolant;

named!(pub mcode<CompleteByteSlice, MCode>, alt!(
    pause |
    user_command |
    end_program |
    spindle_rotation |
    tool_change |
    modal_state |
    coolant
));

/// Enum describing all supported M-codes
#[derive(Debug, PartialEq)]
pub enum MCode {
    Coolant(Coolant),
    EndProgram,
    Pause,
    OptionalPause,
    SpindleRotation(SpindleRotation),
    ToolChange,
    UserCommand(u32),
    ModalStateSave,
    ModalStateInvalidate,
    ModalStateRestore,
    ModalStateAutoRestore,
}
