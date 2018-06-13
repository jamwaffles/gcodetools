#[macro_use]
mod macros;

mod g0_rapid_move;
mod g17_g19_plane_select;
mod g1_linear_move;
mod g20_g21_units;
mod g28_g30_predefined_position;
mod g2_g3_arc;
mod g33_spindle_sync_motion;
mod g38_straight_probe;
mod g40_g41_g42_cutter_compensation;
mod g43_g49_tool_length_compensation;
mod g4_dwell;
mod g53_global_move;
mod g54_g55_work_offset;
mod g61_g64_path_blending;
mod g7_g8_lathe_measurement_mode;
mod g80_canned_cycle;
mod g90_g91_distance_mode;
mod g92_coordinate_system_offset;
mod g93_g95_feedrate_mode;

use super::Token;
use nom::types::CompleteByteSlice;

use self::g0_rapid_move::rapid_move;
use self::g17_g19_plane_select::plane_select;
use self::g1_linear_move::linear_move;
use self::g20_g21_units::units;
use self::g28_g30_predefined_position::predefined_position;
use self::g2_g3_arc::arc;
use self::g33_spindle_sync_motion::spindle_sync_motion;
use self::g38_straight_probe::straight_probe;
use self::g40_g41_g42_cutter_compensation::cutter_compensation;
use self::g43_g49_tool_length_compensation::tool_length_compensation;
use self::g4_dwell::dwell;
use self::g53_global_move::global_move;
use self::g54_g55_work_offset::work_offset;
use self::g61_g64_path_blending::path_blending;
use self::g7_g8_lathe_measurement_mode::lathe_measurement_mode;
use self::g80_canned_cycle::canned_cycle;
use self::g90_g91_distance_mode::distance_mode;
use self::g92_coordinate_system_offset::coordinate_system_offset;
use self::g93_g95_feedrate_mode::feedrate_mode;

pub use self::g17_g19_plane_select::Plane;
pub use self::g20_g21_units::Units;
pub use self::g28_g30_predefined_position::PredefinedPosition;
pub use self::g33_spindle_sync_motion::SpindleSyncMotion;
pub use self::g38_straight_probe::StraightProbe;
pub use self::g40_g41_g42_cutter_compensation::CutterCompensation;
pub use self::g43_g49_tool_length_compensation::ToolLengthCompensation;
pub use self::g54_g55_work_offset::WorkOffset;
pub use self::g61_g64_path_blending::PathBlendingMode;
pub use self::g7_g8_lathe_measurement_mode::LatheMeasurementMode;
pub use self::g90_g91_distance_mode::DistanceMode;
pub use self::g93_g95_feedrate_mode::FeedrateMode;

named!(pub gcode<CompleteByteSlice, Token>, alt!(
    rapid_move |
    plane_select |
    linear_move |
    units |
    predefined_position |
    arc |
    straight_probe |
    cutter_compensation |
    tool_length_compensation |
    dwell |
    global_move |
    work_offset |
    path_blending |
    lathe_measurement_mode |
    canned_cycle |
    distance_mode |
    coordinate_system_offset |
    feedrate_mode |
    spindle_sync_motion
));
