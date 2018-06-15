use nom::types::CompleteByteSlice;

use super::MCode;

named!(pub end_program<CompleteByteSlice, MCode>, alt!(
    m_code!("30", MCode::EndProgram) |
    m_code!("2", MCode::EndProgram)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_end_program() {
        assert_complete_parse!(end_program(Cbs(b"M2")), MCode::EndProgram);
        assert_complete_parse!(end_program(Cbs(b"M30")), MCode::EndProgram);
    }
}
