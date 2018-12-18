use nom::types::CompleteByteSlice;

use super::MCode;

named!(pub modal_state<CompleteByteSlice, MCode>, alt!(
    m_code!("70", MCode::ModalStateSave) |
    m_code!("71", MCode::ModalStateInvalidate) |
    m_code!("72", MCode::ModalStateRestore) |
    m_code!("73", MCode::ModalStateAutoRestore)
));
