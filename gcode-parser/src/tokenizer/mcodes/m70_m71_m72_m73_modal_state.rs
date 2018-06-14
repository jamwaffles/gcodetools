use nom::types::CompleteByteSlice;

use super::super::Token;

named!(pub modal_state<CompleteByteSlice, Token>, alt!(
    m_int!(70, Token::ModalStateSave) |
    m_int!(71, Token::ModalStateInvalidate) |
    m_int!(72, Token::ModalStateRestore) |
    m_int!(73, Token::ModalStateAutoRestore)
));
