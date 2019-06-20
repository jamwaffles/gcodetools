mod gcode;

#[cfg(test)]
mod tests {
    use super::gcode::gcode_expression;
    use crate::{ArithmeticOperator, Expression, ExpressionToken, Function};
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

        let (remaining, result) =
            gcode_expression::<VerboseError<&str>>(expr).map_err(|e| match e {
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
