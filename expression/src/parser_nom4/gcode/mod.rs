//! Parse expressions in the LinuxCNC GCode format

pub mod expression;
mod helpers;
pub mod parameter;
pub mod value;

#[cfg(test)]
mod tests {
    use super::expression::expression as gcode_expression;
    use crate::{ArithmeticOperator, Expression, ExpressionToken, Function};
    use common::{assert_parse, span};

    #[test]
    fn parse_complex_expression() {
        let expr = r#"[ 1 + 2 / 3 * 4 - 5 + sin[5 + 6 * [cos[4] + 2.0 ] ] ]"#;

        let expd = Expression::from_tokens(vec![
            ExpressionToken::Literal(1.0),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
            ExpressionToken::Literal(2.0),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Div),
            ExpressionToken::Literal(3.0),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
            ExpressionToken::Literal(4.0),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
            ExpressionToken::Literal(5.0),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
            ExpressionToken::Function(Function::Sin(
                vec![
                    ExpressionToken::Literal(5.0),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                    ExpressionToken::Literal(6.0),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                    ExpressionToken::Expression(
                        vec![
                            ExpressionToken::Function(Function::Cos(
                                vec![ExpressionToken::Literal(4.0)].into(),
                            )),
                            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                            ExpressionToken::Literal(2.0),
                        ]
                        .into(),
                    ),
                ]
                .into(),
            )),
        ]);

        assert_parse!(
            parser = gcode_expression;
            input = span!(expr.as_bytes());
            expected = expd;
        );
    }
}
