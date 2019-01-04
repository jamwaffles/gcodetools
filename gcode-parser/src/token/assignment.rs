use common::parsing::Span;
use expression::parser::{gcode_parameter, ngc_float_value};
use expression::{Parameter, Value};
use nom::*;

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

named!(pub assignment<Span, Assignment>,
    map!(
        sep!(
            space0,
            separated_pair!(
                gcode_parameter,
                char!('='),
                ngc_float_value
            )
        ),
        |(lhs, rhs)| Assignment { lhs, rhs }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};
    use expression::{ArithmeticOperator, Expression, ExpressionToken};

    #[test]
    fn parse_assignment() {
        assert_parse!(
            parser = assignment;
            input =
                span!(b"#1000 = 1.0"),
                span!(b"#<named> = [1 + 2]")
            ;
            expected =
                Assignment {
                    lhs: Parameter::Numbered(1000),
                    rhs: Value::Float(1.0)
                },
                Assignment {
                    lhs: Parameter::Named("named".into()),
                    rhs: Value::Expression(Expression::from_tokens(vec![
                        ExpressionToken::Literal(1.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Literal(2.0),
                    ]))
                }
            ;
        );
    }
}
