use nom::types::CompleteByteSlice;

use super::GCode;

named!(pub arc<CompleteByteSlice, GCode>, alt!(
    g_code!("2", GCode::ClockwiseArc) |
    g_code!("3", GCode::CounterclockwiseArc)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_arcs() {
        assert_complete_parse!(arc(Cbs(b"G2")), GCode::ClockwiseArc);
        assert_complete_parse!(arc(Cbs(b"G02")), GCode::ClockwiseArc);
        assert_complete_parse!(arc(Cbs(b"G3")), GCode::CounterclockwiseArc);
        assert_complete_parse!(arc(Cbs(b"G03")), GCode::CounterclockwiseArc);
    }
}
