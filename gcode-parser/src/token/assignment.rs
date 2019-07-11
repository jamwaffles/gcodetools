use crate::value::{decimal_value, Value};
use expression::{parser::gcode, Expression, ExpressionToken, Parameter};
use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::map,
    error::{context, ParseError},
    sequence::{delimited, separated_pair},
    IResult,
};

/// Assign a value to a variable
///
/// A value can be a literal or a complete expression
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    /// The parameter to assign a value to
    lhs: Parameter,

    /// The value or result of an expression to assign
    rhs: Value,
}

pub fn assignment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Assignment, E> {
    context(
        "assignment",
        map(
            separated_pair(
                gcode::parameter,
                delimited(space0, char('='), space0),
                alt((
                    decimal_value,
                    map(gcode::function, |f| {
                        Value::Expression(Expression::from_tokens(vec![ExpressionToken::Function(
                            f,
                        )]))
                    }),
                )),
            ),
            |(lhs, rhs)| Assignment { lhs, rhs },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use expression::{ArithmeticOperator, Expression, ExpressionToken, Function};

    #[test]
    fn parse_assignment() {
        assert_parse!(
            parser = assignment;
            input =
                "#1000 = 1.0",
                "#<named> = [1 + 2]"
            ;
            expected =
                Assignment {
                    lhs: Parameter::Numbered(1000),
                    rhs: 1.0.into()
                },
                Assignment {
                    lhs: Parameter::Local("named".into()),
                    rhs: Value::Expression(Expression::from_tokens(vec![
                        ExpressionToken::Literal(1.0.into()),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Literal(2.0.into()),
                    ]))
                }
            ;
        );
    }

    #[test]
    fn parse_function_assignment() {
        assert_parse!(
            parser = assignment;
            input =
                "#1000 = sin[1.0]"
            ;
            expected =
                Assignment {
                    lhs: Parameter::Numbered(1000),
                    rhs: Value::Expression(Expression::from_tokens(vec![
                        ExpressionToken::Function(Function::Sin(Expression::from_tokens(vec![
                            ExpressionToken::Literal(1.0.into()),
                        ]))),
                    ]))
                }
            ;
        );
    }
}
