use nom::{
    character::complete::space0,
    combinator::map,
    error::{context, ParseError},
    number::complete::float,
    sequence::separated_pair,
    IResult,
};

/// TODO: Replace with an enum that holds expressions and parameters as well as literals
pub type Value = f32;

/// TODO: Parse expressions and parameters (not surrounded by `[]`) along with literals into an enum
/// TODO: Decide whether to just use `float` from Nom or aim for parity with LinuxCNC's subset
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
    map(separated_pair(parser, space0, value), |(_char, value)| {
        value
    })
}
