use crate::map_code;
use common::parsing::Span;
use nom::*;

// TODO: Better name than WorkOffsetValue
/// Which work offset to use
#[derive(Debug, PartialEq, Clone)]
pub enum WorkOffsetValue {
    /// Offset 0, `G54`
    G54 = 0,
    /// Offset 1, `G55`
    G55 = 1,
    /// Offset 2, `G56`
    G56 = 2,
    /// Offset 3, `G57`
    G57 = 3,
    /// Offset 4, `G58`
    G58 = 4,
    /// Offset 5, `G59`
    G59 = 5,
    /// Offset 6, `G59.1`
    G59_1 = 6,
    /// Offset 7, `G59.2`
    G59_2 = 7,
    /// Offset 8, `G59.3`
    G59_3 = 8,
}

/// Work offset
#[derive(Debug, PartialEq, Clone)]
pub struct WorkOffset {
    /// The type of work offset (`G54`, `G59.1`, etc)
    pub offset: WorkOffsetValue,
}

named!(pub work_offset<Span, WorkOffset>,
    alt_complete!(
        map_code!("G59.1", |_| WorkOffset { offset: WorkOffsetValue::G59_1 }) |
        map_code!("G59.2", |_| WorkOffset { offset: WorkOffsetValue::G59_2 }) |
        map_code!("G59.3", |_| WorkOffset { offset: WorkOffsetValue::G59_3 }) |
        map_code!("G54", |_| WorkOffset { offset: WorkOffsetValue::G54 }) |
        map_code!("G55", |_| WorkOffset { offset: WorkOffsetValue::G55 }) |
        map_code!("G56", |_| WorkOffset { offset: WorkOffsetValue::G56 }) |
        map_code!("G57", |_| WorkOffset { offset: WorkOffsetValue::G57 }) |
        map_code!("G58", |_| WorkOffset { offset: WorkOffsetValue::G58 }) |
        map_code!("G59", |_| WorkOffset { offset: WorkOffsetValue::G59 })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, assert_parse_ok, empty_span, span};

    #[test]
    fn parse_integer_work_offset() {
        let raw = span!(b"G54");

        assert_parse!(
            parser = work_offset;
            input = raw;
            expected = WorkOffset {
                offset: WorkOffsetValue::G54
            }
        );
    }

    #[test]
    fn parse_decimal_work_offset() {
        let raw = span!(b"G59.1");

        assert_parse!(
            parser = work_offset;
            input = raw;
            expected = WorkOffset {
                offset: WorkOffsetValue::G59_1
            }
        );
    }
}
