use nom::types::CompleteByteSlice;

use super::super::helpers::*;
use super::super::Token;

named!(pub modal_state<CompleteByteSlice, Token>, alt_complete!(
    map!(call!(m, 70.0), |_| Token::ModalStateSave) |
    map!(call!(m, 71.0), |_| Token::ModalStateInvalidate) |
    map!(call!(m, 72.0), |_| Token::ModalStateRestore) |
    map!(call!(m, 73.0), |_| Token::ModalStateAutoRestore)
));
