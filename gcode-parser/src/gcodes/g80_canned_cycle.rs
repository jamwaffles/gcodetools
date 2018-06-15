use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub canned_cycle<CompleteByteSlice, GCode>,
    g_code!("80", GCode::CancelCannedCycle)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_canned_cycles() {
        assert_complete_parse!(canned_cycle(Cbs(b"G80")), GCode::CancelCannedCycle);
    }
}
