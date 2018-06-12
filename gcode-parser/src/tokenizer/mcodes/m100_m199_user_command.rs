use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

named!(pub user_command<CompleteByteSlice, Token>, map!(
    call!(preceded_code_range_inclusive, 'M', 100.0, 199.0),
    |(_, command_number)| Token::UserCommand(command_number as u32)
));
