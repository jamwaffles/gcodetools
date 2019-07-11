use expression::{gcode, Expression, ExpressionToken, Parameter};
use nom::{
    branch::alt,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
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
        s.parse::<f32>().map(|v| Value::Literal(v)).or_else(|_| {
            gcode::expression::<(), f32>(s)
                .map(|(_i, e)| Value::Expression(e))
                .map_err(|_e| format!("Expression parse failed"))
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
}

impl From<u32> for UnsignedValue {
    fn from(other: u32) -> Self {
        UnsignedValue::Literal(other)
    }
}

/// TODO: Parse expressions and parameters (not surrounded by `[]`) along with literals into an enum
/// TODO: Decide whether to just use `float` from Nom or aim for parity with LinuxCNC's subset
pub fn decimal_value<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Value, E> {
    context(
        "decimal value",
        alt((
            map(float, |f| Value::Literal(f)),
            map(gcode::parameter, |p| Value::Parameter(p)),
            map(gcode::expression, |e| Value::Expression(e)),
            map(gcode::function, |f| {
                Value::Expression(Expression::from_tokens(vec![ExpressionToken::Function(f)]))
            }),
        )),
    )(i)
}

pub fn unsigned_value<'a, E: ParseError<&'a str>, V>(i: &'a str) -> IResult<&'a str, V, E>
where
    V: From<u32>,
{
    context(
        "unsigned value",
        map_res::<_, _, _, _, String, _, _>(digit1, |n: &'a str| {
            Ok(V::from(n.parse::<u32>().map_err(|e| e.to_string())?))
        }),
    )(i)
}

// TODO: any_value where the value can be floating, integer or an expr
// TODO: integer_value where value can only be integer or expr

/// Parse a value after a preceding parser, separated by 0 or more spaces
pub fn preceded_decimal_value<'a, P, OP, E: ParseError<&'a str>>(
    parser: P,
) -> impl Fn(&'a str) -> IResult<&'a str, Value, E>
where
    P: Fn(&'a str) -> IResult<&'a str, OP, E>,
{
    // TODO: Benchmark against impl below
    // map(preceded(terminated(parser, space0), value), |value| value)

    map(
        separated_pair(parser, space0, decimal_value),
        |(_char, value)| value,
    )
}

pub fn preceded_unsigned_value<'a, P, OP, E: ParseError<&'a str>, V>(
    parser: P,
) -> impl Fn(&'a str) -> IResult<&'a str, V, E>
where
    P: Fn(&'a str) -> IResult<&'a str, OP, E>,
    V: From<u32>,
{
    // TODO: Benchmark against impl below
    // map(preceded(terminated(parser, space0), value), |value| value)

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
                            ExpressionToken::Literal(1.234.into()),
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
