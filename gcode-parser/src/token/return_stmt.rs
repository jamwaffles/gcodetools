use common::parsing::Span;
use expression::parser::gcode_expression;
use expression::Expression;
use nom::*;

/// Which type of block this is
#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    value: Option<Expression>,
}

named!(pub return_stmt<Span, Return>,
    sep!(
        space0,
        map!(
            preceded!(
                tag_no_case!("return"),
                opt!(gcode_expression)
            ),
            |value| {
                Return { value }
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};
    use expression::{ArithmeticOperator, Expression, ExpressionToken};

    #[test]
    fn parse_return() {
        assert_parse!(
            parser = return_stmt;
            input = span!(b"return [1 + 2]");
            expected = Return {
                value: Some(Expression::from_tokens(vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                    ExpressionToken::Literal(2.0),
                ]))
            };
        );
    }

    #[test]
    fn parse_return_no_value() {
        assert_parse!(
            parser = return_stmt;
            input = span!(b"return");
            expected = Return {
                value: None
            };
        );
    }
}
