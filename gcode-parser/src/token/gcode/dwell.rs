use crate::parsers::char_no_case;
use crate::value::{preceded_decimal_value, Value};
use crate::word::word;
use nom::{
    character::complete::space0,
    combinator::map,
    error::{context, ParseError},
    sequence::separated_pair,
    IResult,
};

/// Dwell
#[derive(Debug, PartialEq, Clone)]
pub struct Dwell {
    /// The length of time in seconds to dwell for
    pub time: Value,
}

pub fn dwell<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Dwell, E> {
    context(
        "dwell",
        map(
            separated_pair(
                word("g4"),
                space0,
                preceded_decimal_value(char_no_case('p')),
            ),
            |(_, time)| Dwell { time },
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
