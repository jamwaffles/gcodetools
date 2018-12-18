use nom::types::CompleteByteSlice;

use super::MCode;

named!(pub pause<CompleteByteSlice, MCode>, alt!(
    m_code!("0", MCode::Pause) |
    m_code!("1", MCode::OptionalPause)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_pauses() {
        assert_complete_parse!(pause(Cbs(b"M0")), MCode::Pause);
        assert_complete_parse!(pause(Cbs(b"M00")), MCode::Pause);

        assert_complete_parse!(pause(Cbs(b"M1")), MCode::OptionalPause);
        assert_complete_parse!(pause(Cbs(b"M01")), MCode::OptionalPause);
    }
}
