use crate::value::{preceded_decimal_value, Value};
use crate::word::word;
use nom::{
    bytes::complete::tag_no_case,
    character::complete::space0,
    combinator::map,
    error::{context, ParseError},
    sequence::{pair, preceded},
    IResult,
};

/// Dwell
#[derive(Debug, PartialEq, Clone)]
pub struct Dwell {
    /// The length of time in seconds to dwell for
    pub time: Value,
}

// named!(pub dwell<Span, Dwell>,
//     map_code!(
//         "G4",
//         preceded!(
//             char_no_case!('P'),
//             ngc_float_value
//         ),
//         |time| Dwell { time }
//     )
// );

pub fn dwell<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Dwell, E> {
    context(
        "dwell",
        map(
            preceded(
                pair(word("g4"), space0),
                preceded_decimal_value(tag_no_case("p")),
            ),
            |time| Dwell { time },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use nom::error::VerboseError;

    #[test]
    fn dwell_decimal() {
        assert_parse!(
            parser = dwell;
            input = "G4 P0.01";
            expected = Dwell { time: 0.01.into() }
        );
    }

    #[test]
    fn leading_zero() {
        assert_parse!(
            parser = dwell;
            input = "G04P3";
            expected = Dwell { time: 3.0.into() }
        );
    }

    #[test]
    fn dwell_integer() {
        assert_parse!(
            parser = dwell;
            input = "G4 P3";
            expected = Dwell { time: 3.0.into() }
        );
    }

    #[test]
    #[should_panic]
    fn dwell_p_value_required() {
        dwell::<VerboseError<&str>>("G4").unwrap();
    }
}
