use crate::value::{preceded_value, Value};
use nom::{
    bytes::streaming::tag_no_case,
    character::streaming::multispace0,
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
                pair(tag_no_case("g4"), multispace0),
                preceded_value(tag_no_case("p")),
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
