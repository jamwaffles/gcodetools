use crate::Expression;
use nom::{error::ParseError, IResult};

/// Expression entry point
pub fn gcode_expression_root<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Expression, E> {
    Ok((i, Expression::empty()))
}
