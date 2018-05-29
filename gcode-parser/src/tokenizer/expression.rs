use super::helpers::*;
use nom::types::CompleteByteSlice;
use nom::*;

#[derive(Debug, PartialEq)]
enum ArithmeticOperator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
enum ExpressionToken {
    Expression(Expression),
    Literal(f32),
    ArithmeticOperator(ArithmeticOperator),
}

type Expression = Vec<ExpressionToken>;

named!(literal<CompleteByteSlice, ExpressionToken>, map!(
    flat_map!(recognize_float, parse_to!(f32)),
    |res| ExpressionToken::Literal(res)
));

named!(arithmetic<CompleteByteSlice, ExpressionToken>, map!(
    map_res!(
        one_of!("+-*/"),
        |operator| match operator {
            '+' => Ok(ArithmeticOperator::Add),
            '-' => Ok(ArithmeticOperator::Sub),
            '*' => Ok(ArithmeticOperator::Mul),
            '/' => Ok(ArithmeticOperator::Div),
            _ => Err(())
        }
    ),
    |res| ExpressionToken::ArithmeticOperator(res)
));

named!(expression_token<CompleteByteSlice, ExpressionToken>, alt_complete!(
    literal |
    arithmetic |
    map!(expression, |res| ExpressionToken::Expression(res))
));

named!(expression<CompleteByteSlice, Expression>, ws!(
    delimited!(
        char!('['),
        many1!(expression_token),
        char!(']')
    )
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_expression(
        to_check: Result<(CompleteByteSlice, Expression), nom::Err<CompleteByteSlice>>,
        against: Expression,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_simple_expressions() {
        let input = Cbs(b"[1]");

        check_expression(expression(input), vec![ExpressionToken::Literal(1.0)]);
    }

    #[test]
    fn it_parses_arithmetic() {
        let input = Cbs(b"[1 + 2 * 3 / 4 - 5]");

        check_expression(
            expression(input),
            vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Literal(2.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(3.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Div),
                ExpressionToken::Literal(4.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
                ExpressionToken::Literal(5.0),
            ],
        );
    }

    #[test]
    fn it_parses_nested_expressions() {
        let input = Cbs(b"[1 + [[2 - 3] * 4]]");

        check_expression(
            expression(input),
            vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Expression(vec![
                    ExpressionToken::Expression(vec![
                        ExpressionToken::Literal(2.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
                        ExpressionToken::Literal(3.0),
                    ]),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                    ExpressionToken::Literal(4.0),
                ]),
            ],
        );
    }

    //     #[test]
    //     fn it_parses_expressions_with_parameters() {
    //         let input = Cbs(b"[1 + #1234 * #<named_param> / #<_global_param>]");
    //     }

    //     #[test]
    //     fn it_parses_function_calls() {
    //         let input = Cbs(b"[SIN[10]]");
    //     }
}
