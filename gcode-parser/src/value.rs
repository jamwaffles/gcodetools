use nom::{
    character::complete::space0,
    combinator::map,
    error::{context, ParseError},
    number::complete::float,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

/// TODO: Replace with an enum that holds expressions and parameters as well as literals
pub type Value = f32;

/// TODO: Parse expressions and parameters (not surrounded by `[]`) along with literals into an enum
/// TODO: Decide whether to just use `float` from Nom or aim for parity with LinuxCNC's subset
/// TODO: Bench with `lexical` feature on or off
pub fn value<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Value, E> {
    context("value", float)(i)
}

/// Parse a value after a preceding parser, separated by 0 or more spaces
pub fn preceded_value<'a, P, OP, E: ParseError<&'a str>>(
    parser: P,
) -> impl Fn(&'a str) -> IResult<&'a str, Value, E>
where
    P: Fn(&'a str) -> IResult<&'a str, OP, E>,
{
    // TODO: Benchmark against impl below
    // map(preceded(terminated(parser, space0), value), |value| value)

    map(separated_pair(parser, space0, value), |(_char, value)| {
        value
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag_no_case;

    #[test]
    fn float_trailing_spaces() {
        assert_parse!(
            parser = value;
            input = "1.234  ";
            expected = 1.234.into();
            remaining = "  ";
        );
    }

    #[test]
    fn preceded_value_spaces() {
        let p = preceded_value(tag_no_case("G"));

        assert_parse!(
            parser = p;
            input = "G 1.234  ";
            expected = 1.234.into();
            remaining = "  ";
        );
    }
}
