use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::MCode;

named!(pub user_command<CompleteByteSlice, MCode>, map!(
    call!(preceded_code_range_inclusive, "M", 100, 199),
    |command_number| MCode::UserCommand(command_number)
));
