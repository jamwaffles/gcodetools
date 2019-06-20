use crate::{
    ArithmeticOperator, BinaryOperator, Expression, ExpressionToken, Function, LogicalOperator,
    Parameter, Value,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until},
    character::complete::{alphanumeric1, char, multispace0},
    combinator::{map, map_res, not, opt, recognize},
    error::{context, ParseError},
    multi::many1,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

/// Expression entry point
pub fn gcode_expression<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Expression, E> {
    context(
        "expression",
        map(
            delimited(char('['), many1(expression_token), char(']')),
            Expression::from_tokens,
        ),
    )(i)
}

fn expression_token<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, ExpressionToken, E> {
    delimited(
        multispace0,
        alt((
            map(operator, ExpressionToken::ArithmeticOperator),
            map(logical_operator, ExpressionToken::LogicalOperator),
            map(binary_operator, ExpressionToken::BinaryOperator),
            map(function, ExpressionToken::Function),
            map(exists, ExpressionToken::Function),
            map(gcode_expression, ExpressionToken::Expression),
            map(literal, ExpressionToken::Literal),
        )),
        multispace0,
    )(i)
}

fn literal<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Value, E> {
    alt((
        map_res(
            recognize(delimited(opt(char('-')), alphanumeric1, not(char('.')))),
            |s| String::from(s).parse::<i64>().map(Value::Integer),
        ),
        map(double, Value::Double),
    ))(i)
}

fn operator<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ArithmeticOperator, E> {
    alt((
        map(char('/'), |_| ArithmeticOperator::Div),
        map(char('*'), |_| ArithmeticOperator::Mul),
        map(char('+'), |_| ArithmeticOperator::Add),
        map(char('-'), |_| ArithmeticOperator::Sub),
        map(tag_no_case("mod"), |_| ArithmeticOperator::Mod),
    ))(i)
}

fn logical_operator<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, LogicalOperator, E> {
    alt((
        map(tag("AND"), |_| LogicalOperator::And),
        map(tag("OR"), |_| LogicalOperator::Or),
        map(tag("NOT"), |_| LogicalOperator::Not),
    ))(i)
}

fn binary_operator<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, BinaryOperator, E> {
    alt((
        map(tag_no_case("EQ"), |_| BinaryOperator::Equal),
        map(tag_no_case("NE"), |_| BinaryOperator::NotEqual),
        map(tag_no_case("GT"), |_| BinaryOperator::GreaterThan),
        map(tag_no_case("GE"), |_| BinaryOperator::GreaterThanOrEqual),
        map(tag_no_case("LT"), |_| BinaryOperator::LessThan),
        map(tag_no_case("LE"), |_| BinaryOperator::LessThanOrEqual),
    ))(i)
}

fn parameter<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Parameter, E> {
    context(
        "parameter",
        preceded(
            char('#'),
            alt((
                map(delimited(tag("<_"), take_until(">"), char('>')), |s| {
                    Parameter::Global(String::from(s))
                }),
                map(delimited(char('<'), take_until(">"), char('>')), |s| {
                    Parameter::Local(String::from(s))
                }),
                map_res(alphanumeric1, |s| {
                    String::from(s).parse::<u32>().map(Parameter::Numbered)
                }),
            )),
        ),
    )(i)
}

/// `exists` is a special function that can only take a single parameter as an argument
fn exists<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Function, E> {
    context(
        "exists",
        map(
            preceded(
                tag_no_case("exists"),
                delimited(
                    char('['),
                    delimited(multispace0, parameter, multispace0),
                    char(']'),
                ),
            ),
            Function::Exists,
        ),
    )(i)
}

fn function<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Function, E> {
    context(
        "function",
        alt((
            map(
                preceded(tag_no_case("abs"), gcode_expression),
                Function::Abs,
            ),
            map(
                preceded(tag_no_case("acos"), gcode_expression),
                Function::Acos,
            ),
            map(
                preceded(tag_no_case("asin"), gcode_expression),
                Function::Asin,
            ),
            map(
                preceded(
                    tag_no_case("atan"),
                    separated_pair(gcode_expression, char('/'), gcode_expression),
                ),
                Function::Atan,
            ),
            map(
                preceded(tag_no_case("cos"), gcode_expression),
                Function::Cos,
            ),
            map(
                preceded(tag_no_case("exp"), gcode_expression),
                Function::Exp,
            ),
            map(
                preceded(tag_no_case("floor"), gcode_expression),
                Function::Floor,
            ),
            map(
                preceded(tag_no_case("ceil"), gcode_expression),
                Function::Ceil,
            ),
            map(preceded(tag_no_case("ln"), gcode_expression), Function::Ln),
            map(
                preceded(tag_no_case("round"), gcode_expression),
                Function::Round,
            ),
            map(
                preceded(tag_no_case("sin"), gcode_expression),
                Function::Sin,
            ),
            map(
                preceded(tag_no_case("sqrt"), gcode_expression),
                Function::Sqrt,
            ),
            map(
                preceded(tag_no_case("tan"), gcode_expression),
                Function::Tan,
            ),
        )),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{convert_error, VerboseError},
        Err,
    };

    #[test]
    fn parse_parameters() -> Result<(), String> {
        let numbered = "#123";
        let local = "#<local>";
        let global = "#<_global>";

        let (remaining, result) =
            parameter::<VerboseError<&str>>(numbered).map_err(|e| match e {
                Err::Error(e) | Err::Failure(e) => convert_error(numbered, e),
                _ => String::from("Failed to parse for unknown reason"),
            })?;

        assert_eq!(remaining.len(), 0);
        assert_eq!(result, Parameter::Numbered(123u32));

        let (remaining, result) = parameter::<VerboseError<&str>>(local).map_err(|e| match e {
            Err::Error(e) | Err::Failure(e) => convert_error(local, e),
            _ => String::from("Failed to parse for unknown reason"),
        })?;

        assert_eq!(remaining.len(), 0);
        assert_eq!(result, Parameter::Local("local".into()));

        let (remaining, result) = parameter::<VerboseError<&str>>(global).map_err(|e| match e {
            Err::Error(e) | Err::Failure(e) => convert_error(global, e),
            _ => String::from("Failed to parse for unknown reason"),
        })?;

        assert_eq!(remaining.len(), 0);
        assert_eq!(result, Parameter::Global("global".into()));

        Ok(())
    }
}
