//! Parse expression strings into expression token lists

pub mod gcode;

#[cfg(test)]
mod tests {
    use super::gcode::expression;
    use crate::*;
    use nom::{
        error::{convert_error, VerboseError},
        Err,
    };

    #[test]
    fn parse_complex_expression() -> Result<(), String> {
        let expr = r#"[ 1 + 2 / 3 * 4 - 5 + sin[5 + 6 * [cos[4] + 2.0 ] ] ]"#;

        let expd = Expression::from_tokens(vec![
            ExpressionToken::Literal(1.into()),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
            ExpressionToken::Literal(2.into()),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Div),
            ExpressionToken::Literal(3.into()),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
            ExpressionToken::Literal(4.into()),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
            ExpressionToken::Literal(5.into()),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
            ExpressionToken::Function(Function::Sin(
                vec![
                    ExpressionToken::Literal(5.into()),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                    ExpressionToken::Literal(6.into()),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                    ExpressionToken::Expression(
                        vec![
                            ExpressionToken::Function(Function::Cos(
                                vec![ExpressionToken::Literal(4.into())].into(),
                            )),
                            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                            ExpressionToken::Literal(2.0.into()),
                        ]
                        .into(),
                    ),
                ]
                .into(),
            )),
        ]);

        // TODO: Format error helper function to move into common crate
        let (remaining, result) =
            expression::<VerboseError<&str>, f64>(expr).map_err(|e| match e {
                Err::Error(e) | Err::Failure(e) => {
                    let e = convert_error(expr, e);
                    println!("{}", e);
                    e
                }
                _ => String::from("Failed to parse for unknown reason"),
            })?;

        assert_eq!(remaining.len(), 0);
        assert_eq!(result, expd);

        Ok(())
    }

    #[test]
    fn rectangle_probe_ngc_condition() -> Result<(), String> {
        let expr = "[ [#<in_or_mm> ne 20] and [#<in_or_mm> ne 21]]";

        let expd = Expression::from_tokens(vec![
            ExpressionToken::Expression(Expression::from_tokens(vec![
                ExpressionToken::Parameter(Parameter::Local("in_or_mm".into())),
                ExpressionToken::BinaryOperator(BinaryOperator::NotEqual),
                ExpressionToken::Literal(20.0),
            ])),
            ExpressionToken::LogicalOperator(LogicalOperator::And),
            ExpressionToken::Expression(Expression::from_tokens(vec![
                ExpressionToken::Parameter(Parameter::Local("in_or_mm".into())),
                ExpressionToken::BinaryOperator(BinaryOperator::NotEqual),
                ExpressionToken::Literal(21.0),
            ])),
        ]);

        // TODO: Format error helper function to move into common crate
        let (remaining, result) =
            expression::<VerboseError<&str>, f64>(expr).map_err(|e| match e {
                Err::Error(e) | Err::Failure(e) => {
                    let e = convert_error(expr, e);
                    println!("{}", e);
                    e
                }
                _ => String::from("Failed to parse for unknown reason"),
            })?;

        assert_eq!(remaining.len(), 0);
        assert_eq!(result, expd);

        Ok(())
    }
}
