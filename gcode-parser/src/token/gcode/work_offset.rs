use nom::{
    branch::alt,
    bytes::streaming::{tag, tag_no_case, take_until},
    character::streaming::{char, digit1, multispace0},
    combinator::{map, map_res, opt},
    error::{context, ParseError},
    multi::many1,
    number::streaming::float,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

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

pub fn work_offset<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, WorkOffset, E> {
    context(
        "work offset",
        map(
            alt((
                map(tag_no_case("G59.1"), |_| WorkOffsetValue::G59_1),
                map(tag_no_case("G59.2"), |_| WorkOffsetValue::G59_2),
                map(tag_no_case("G59.3"), |_| WorkOffsetValue::G59_3),
                map(tag_no_case("G54"), |_| WorkOffsetValue::G54),
                map(tag_no_case("G55"), |_| WorkOffsetValue::G55),
                map(tag_no_case("G56"), |_| WorkOffsetValue::G56),
                map(tag_no_case("G57"), |_| WorkOffsetValue::G57),
                map(tag_no_case("G58"), |_| WorkOffsetValue::G58),
                map(tag_no_case("G59"), |_| WorkOffsetValue::G59),
            )),
            |offset| WorkOffset { offset },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;

    #[test]
    fn parse_integer_work_offset() {
        assert_parse!(
            parser = work_offset;
            input = "G54";
            expected = WorkOffset {
                offset: WorkOffsetValue::G54
            }
        );
    }

    #[test]
    fn parse_decimal_work_offset() {
        assert_parse!(
            parser = work_offset;
            input = "G59.1";
            expected = WorkOffset {
                offset: WorkOffsetValue::G59_1
            }
        );
    }
}
