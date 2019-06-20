use crate::{ArithmeticOperator, Expression, ExpressionToken, Value};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, multispace0},
    combinator::{map, opt, recognize},
    error::{context, ParseError},
    multi::many1,
    number::complete::double,
    sequence::{delimited, preceded},
    IResult,
};

/// Expression entry point
pub fn gcode_expression<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Expression, E> {
    // Ok((i, Expression::empty()))
    context(
        "expression",
        delimited(char('['), many1(expression_token), char(']')),
    )(i)
    .map(|(i, _r)| (i, Expression::empty()))
}

fn expression_token<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ExpressionToken, E> {
    delimited(
        multispace0,
        alt((
            map(literal, ExpressionToken::Literal),
            map(operator, ExpressionToken::ArithmeticOperator),
            map(gcode_expression, ExpressionToken::Expression),
        )),
        multispace0,
    )(i)
}

fn literal<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Value, E> {
    // Ok((i, ExpressionToken::Literal(0.0.into())))

    alt((
        map(double, Value::Float),
        map_res(alphanumeric1, |s| {
            String::from(s).parse::<u64>().map(Value::Unsigned)
        }),
        map_res(recognize(preceded(opt(char('-')), alphanumeric1)), |s| {
            String::from(s).parse::<i64>().map(Value::Signed)
        }),
    ))(i)
}

fn operator<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ArithmeticOperator, E> {
    // Ok((
    //     i,
    //     ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
    // ))

    alt((
        map(char('/'), |_| ArithmeticOperator::Div),
        map(char('*'), |_| ArithmeticOperator::Mul),
        map(char('+'), |_| ArithmeticOperator::Add),
        map(char('-'), |_| ArithmeticOperator::Sub),
    ))(i)
}
