use nom::types::CompleteByteSlice;

use super::MCode;

named!(pub modal_state<CompleteByteSlice, MCode>, alt!(
    m_int!(70, MCode::ModalStateSave) |
    m_int!(71, MCode::ModalStateInvalidate) |
    m_int!(72, MCode::ModalStateRestore) |
    m_int!(73, MCode::ModalStateAutoRestore)
));
