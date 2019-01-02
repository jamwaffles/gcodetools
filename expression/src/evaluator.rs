use crate::{ArithmeticOperator, Context, Expression, ExpressionToken, Function};

fn shunting_yard(tokens: Expression, context: Option<&Context>) -> Vec<ExpressionToken> {
    let mut output: Vec<ExpressionToken> = Vec::new();
    let mut operators: Vec<ExpressionToken> = Vec::new();

    for token in tokens {
        match token {
            ExpressionToken::Literal(_) => output.push(token),
            ExpressionToken::ArithmeticOperator(operator) => {
                while let Some(top) = operators.last().map(|value| value.clone()) {
                    match top {
                        ExpressionToken::ArithmeticOperator(top_op) => {
                            if top_op > operator
                            /* || (top_op == operator&& operator.is_left_associative)*/
                            {
                                output.push(operators.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
                operators.push(token);
            }
            ExpressionToken::Expression(nested_expr) => {
                let res = evaluate(nested_expr, context);

                output.push(ExpressionToken::Literal(res.unwrap()));
            }
            ExpressionToken::Parameter(_) => output.push(token),
            ExpressionToken::Function(func) => {
                let res = match func {
                    Function::Abs(arg) => evaluate(arg, context).unwrap().abs(),
                    Function::Acos(arg) => evaluate(arg, context).unwrap().acos(),
                    Function::Asin(arg) => evaluate(arg, context).unwrap().asin(),
                    Function::Atan((arg1, arg2)) => {
                        let res1 = evaluate(arg1, context).unwrap();
                        let res2 = evaluate(arg2, context).unwrap();

                        res1.atan2(res2)
                    }
                    Function::Cos(arg) => evaluate(arg, context).unwrap().cos(),
                    Function::Exists(arg) => match context {
                        Some(ctx) => match ctx.contains_key(&arg) {
                            true => 1.0,
                            false => 0.0,
                        },
                        None => 0.0,
                    },
                    Function::Exp(arg) => evaluate(arg, context).unwrap().exp(),
                    Function::Floor(arg) => evaluate(arg, context).unwrap().floor(),
                    Function::Ceil(arg) => evaluate(arg, context).unwrap().ceil(),
                    Function::Ln(arg) => evaluate(arg, context).unwrap().ln(),
                    Function::Round(arg) => evaluate(arg, context).unwrap().round(),
                    Function::Sin(arg) => evaluate(arg, context).unwrap().sin(),
                    Function::Sqrt(arg) => evaluate(arg, context).unwrap().sqrt(),
                    Function::Tan(arg) => evaluate(arg, context).unwrap().tan(),
                };

                output.push(ExpressionToken::Literal(res))
            }
            _ => unimplemented!(),
        }
    }

    while let Some(token) = operators.pop() {
        output.push(token)
    }

    assert!(operators.is_empty());

    output
}

fn calculate(postfix_tokens: Vec<ExpressionToken>, context: Option<&Context>) -> Result<f32, ()> {
    let mut stack = Vec::new();

    for token in postfix_tokens {
        match token {
            ExpressionToken::Literal(number) => stack.push(number),
            ExpressionToken::ArithmeticOperator(operator) => {
                if let Some(y) = stack.pop() {
                    if let Some(x) = stack.pop() {
                        let result = match operator {
                            ArithmeticOperator::Div => x / y,
                            ArithmeticOperator::Mul => x * y,
                            ArithmeticOperator::Add => x + y,
                            ArithmeticOperator::Sub => x - y,
                        };

                        stack.push(result);

                        continue;
                    }
                }
            }
            ExpressionToken::Parameter(param) => {
                let value = context.unwrap().get(&param).unwrap();

                stack.push(*value)
            }
            _ => unimplemented!(),
        }
    }

    assert!(stack.len() == 1);

    Ok(stack.pop().unwrap())
}

// TODO: Some way of returning `T` instead of a rigid `f32`
// TODO: Better error than `()`
/// Evaluate an expression with an optional context object
pub fn evaluate(expression: Expression, context: Option<&Context>) -> Result<f32, ()> {
    let postfix_tokens = shunting_yard(expression, context);

    calculate(postfix_tokens, context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parameter;

    macro_rules! assert_near {
        ($compare:expr, $expected:expr) => {
            assert!(
                ($compare - $expected).abs() < 0.000001,
                format!("{:?} not near to {:?}", $compare, $expected)
            );
        };
    }

    #[test]
    fn it_evaluates_simple_expressions() {
        let expr = vec![
            ExpressionToken::Literal(1.0),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
            ExpressionToken::Literal(2.0),
        ];

        assert_eq!(evaluate(expr, None), Ok(3.0));
    }

    #[test]
    fn it_evaluates_nested_expressions() {
        let expr = vec![
            ExpressionToken::Literal(1.0),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
            ExpressionToken::Expression(vec![
                ExpressionToken::Literal(2.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Literal(3.0),
            ]),
        ];

        assert_eq!(evaluate(expr, None), Ok(6.0));
    }

    #[test]
    fn it_evaluates_parameters() {
        let expr = vec![
            ExpressionToken::Parameter(Parameter::Numbered(1234)),
            ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
            ExpressionToken::Expression(vec![
                ExpressionToken::Parameter(Parameter::Named("named".into())),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Parameter(Parameter::Global("global".into())),
            ]),
        ];

        let context: Context = hashmap! {
            Parameter::Numbered(1234) => 1.2,
            Parameter::Named("named".into()) => 3.4,
            Parameter::Global("global".into()) => 5.6,
        };

        assert_eq!(evaluate(expr, Some(&context)), Ok(10.2));
    }

    #[test]
    fn it_evaluates_exists() {
        let good_ctx: Context = hashmap! {
            Parameter::Named("foo_bar".into()) => 1.0,
        };
        let bad_ctx: Context = hashmap! {
            Parameter::Named("baz_quux".into()) => 1.0,
            Parameter::Global("foo_bar".into()) => 1.0,
        };

        assert_eq!(
            evaluate(
                vec![ExpressionToken::Function(Function::Exists(
                    Parameter::Named("foo_bar".into()),
                ))],
                Some(&good_ctx)
            ),
            Ok(1.0)
        );

        assert_eq!(
            evaluate(
                vec![ExpressionToken::Function(Function::Exists(
                    Parameter::Named("foo_bar".into()),
                ))],
                Some(&bad_ctx)
            ),
            Ok(0.0)
        );

        assert_eq!(
            evaluate(
                vec![ExpressionToken::Function(Function::Exists(
                    Parameter::Named("foo_bar".into()),
                ))],
                None
            ),
            Ok(0.0)
        );
    }

    #[test]
    fn it_evaluates_atan() {
        let atan = vec![ExpressionToken::Function(Function::Atan((
            vec![ExpressionToken::Literal(1.0)],
            vec![ExpressionToken::Literal(2.0)],
        )))];

        assert_eq!(evaluate(atan, None), Ok(0.4636476));
    }

    // Not an exhaustive test by any means, but it should get us in the ballpark
    #[test]
    fn it_evaluates_functions() {
        let funcs: Vec<(Function, f32)> = vec![
            (Function::Abs(vec![ExpressionToken::Literal(-1.5)]), 1.5),
            (Function::Acos(vec![ExpressionToken::Literal(1.0)]), 0.0),
            (
                Function::Asin(vec![ExpressionToken::Literal(1.0)]),
                1.5707964,
            ),
            (
                Function::Cos(vec![ExpressionToken::Literal(1.0)]),
                0.5403023,
            ),
            (
                Function::Exp(vec![ExpressionToken::Literal(1.0)]),
                2.7182817,
            ),
            (Function::Floor(vec![ExpressionToken::Literal(2.8)]), 2.0),
            (Function::Floor(vec![ExpressionToken::Literal(-2.8)]), -3.0),
            (Function::Ceil(vec![ExpressionToken::Literal(2.8)]), 3.0),
            (Function::Ceil(vec![ExpressionToken::Literal(-2.8)]), -2.0),
            (Function::Ln(vec![ExpressionToken::Literal(2.0)]), 0.6931472),
            (Function::Round(vec![ExpressionToken::Literal(1.4)]), 1.0),
            (Function::Round(vec![ExpressionToken::Literal(1.5)]), 2.0),
            (Function::Round(vec![ExpressionToken::Literal(1.6)]), 2.0),
            (
                Function::Sin(vec![ExpressionToken::Literal(1.0)]),
                0.84147096,
            ),
            (
                Function::Sqrt(vec![ExpressionToken::Literal(3.0)]),
                1.7320508,
            ),
            (
                Function::Tan(vec![ExpressionToken::Literal(1.0)]),
                1.5574077,
            ),
        ];

        for (func, expected) in funcs {
            assert_near!(
                evaluate(vec![ExpressionToken::Function(func.clone())], None).unwrap(),
                expected
            );
        }
    }
}
