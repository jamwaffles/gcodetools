use super::super::tokenizer::prelude::*;
use std::collections::HashMap;
use tokenizer::expression::*;

pub type Context = HashMap<Parameter, f32>;

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
            // Behaves the same as a literal; a number is produced
            ExpressionToken::Expression(nested_expr) => {
                let res = evaluate(nested_expr, context);

                output.push(ExpressionToken::Literal(res.unwrap()));
            }
            ExpressionToken::Parameter(_) => output.push(token),
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

pub fn evaluate(expression: Expression, context: Option<&Context>) -> Result<f32, ()> {
    let postfix_tokens = shunting_yard(expression, context);

    calculate(postfix_tokens, context)
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let context: Context = hashmap!{
            Parameter::Numbered(1234) => 1.2,
            Parameter::Named("named".into()) => 3.4,
            Parameter::Global("global".into()) => 5.6,
        };

        assert_eq!(evaluate(expr, Some(&context)), Ok(10.2));
    }
}
