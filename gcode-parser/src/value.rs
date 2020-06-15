use expression::{gcode, Expression, ExpressionToken, Parameter};
use nom::{
    branch::alt,
    character::complete::{digit1, space0},
    combinator::{map, map_res, verify},
    error::{context, ParseError},
    number::complete::float,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

// TODO: Feature for double precision/size (*32 -> *64)
/// Any possible valid floating point value (positive or negative)
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A floating point literal
    Literal(f32),

    /// A GCode expression
    Expression(Expression<f32>),

    /// A parameter like `#3`
    Parameter(Parameter),
}

impl Value {
    /// Convert value to an f64. If `self` is not a `Value`, this method will panic
    ///
    /// This is mostly for testing and should not be used in critical code
    pub fn as_f64_unchecked(&self) -> f64 {
        match self {
            Value::Literal(v) => *v as f64,
            _ => panic!("Value must be a literal"),
        }
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f32>().map(Value::Literal).or_else(|_| {
            gcode::expression::<(), f32>(s)
                .map(|(_i, e)| Value::Expression(e))
                .map_err(|_e| "Expression parse failed".to_string())
        })
    }
}

impl From<f32> for Value {
    fn from(other: f32) -> Self {
        Value::Literal(other)
    }
}

impl From<Parameter> for Value {
    fn from(other: Parameter) -> Self {
        Value::Parameter(other)
    }
}

/// Positive integer only, or an expression that evaluates to one
#[derive(Debug, Clone, PartialEq)]
pub enum UnsignedValue {
    /// Unsigned integer
    Literal(u32),

    /// A GCode expression
    Expression(Expression<u32>),

    /// A parameter (variable)
    Parameter(Parameter),
}

impl From<u32> for UnsignedValue {
    fn from(other: u32) -> Self {
        UnsignedValue::Literal(other)
    }
}

pub fn decimal_value<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Value, E> {
    context(
        "decimal value",
        alt((
            map(float, Value::Literal),
            map(gcode::parameter, Value::Parameter),
            map(gcode::expression, Value::Expression),
            map(gcode::function, |f| {
                Value::Expression(Expression::from_tokens(vec![ExpressionToken::Function(f)]))
            }),
        )),
    )(i)
}

pub fn positive_decimal_value<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Value, E> {
    context(
        "positive-only decimal value",
        verify(decimal_value, |v| match v {
            Value::Literal(n) => *n >= 0.0,
            _ => true,
        }),
    )(i)
}

pub fn unsigned_value<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, UnsignedValue, E> {
    context(
        "unsigned value",
        alt((
            map_res::<_, _, _, _, String, _, _>(digit1, |n: &'a str| {
                Ok(UnsignedValue::Literal(
                    n.parse::<u32>().map_err(|e| e.to_string())?,
                ))
            }),
            map(gcode::parameter, UnsignedValue::Parameter),
            map(gcode::expression, UnsignedValue::Expression),
            map(gcode::function, |f| {
                UnsignedValue::Expression(Expression::from_tokens(vec![ExpressionToken::Function(
                    f,
                )]))
            }),
        )),
    )(i)
}

/// Parse a value after a preceding parser, separated by 0 or more spaces
pub fn preceded_decimal_value<'a, P, OP, E: ParseError<&'a str>>(
    parser: P,
) -> impl Fn(&'a str) -> IResult<&'a str, Value, E>
where
    P: Fn(&'a str) -> IResult<&'a str, OP, E>,
{
    map(
        separated_pair(parser, space0, decimal_value),
        |(_char, value)| value,
    )
}

pub fn preceded_positive_decimal_value<'a, P, OP, E: ParseError<&'a str>>(
    parser: P,
) -> impl Fn(&'a str) -> IResult<&'a str, Value, E>
where
    P: Fn(&'a str) -> IResult<&'a str, OP, E>,
{
    map(
        separated_pair(parser, space0, positive_decimal_value),
        |(_char, value)| value,
    )
}

pub fn preceded_unsigned_value<'a, P, OP, E: ParseError<&'a str>>(
    parser: P,
) -> impl Fn(&'a str) -> IResult<&'a str, UnsignedValue, E>
where
    P: Fn(&'a str) -> IResult<&'a str, OP, E>,
{
    map(
        separated_pair(parser, space0, unsigned_value),
        |(_char, value)| value,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::char_no_case;
    use expression::Function;

    #[test]
    fn float_trailing_spaces() {
        assert_parse!(
            parser = decimal_value;
            input = "1.234  ";
            expected = 1.234.into();
            remaining = "  ";
        );
    }

    #[test]
    fn preceded_decimal_value_spaces() {
        let p = preceded_decimal_value(char_no_case('G'));

        assert_parse!(
            parser = p;
            input = "G 1.234  ";
            expected = 1.234.into();
            remaining = "  ";
        );
    }

    #[test]
    fn function_as_expression() {
        assert_parse!(
            parser = decimal_value;
            input = "SIN[1.234]";
            expected = Value::Expression(
                Expression::from_tokens(vec![
                    ExpressionToken::Function(Function::Sin(
                        vec![
                            ExpressionToken::Literal(1.234),
                        ]
                        .into(),
                    )),
                ])
            );
        );
    }

    #[test]
    fn preceded_decimal_value_parameter() {
        let p = preceded_decimal_value(char_no_case('P'));

        assert_parse!(
            parser = p;
            input = "p #<g64tol> ; path tolerance";
            expected = Parameter::Local("g64tol".to_string()).into();
            remaining = " ; path tolerance";
        );
    }
}
