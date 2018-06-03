use nom::types::CompleteByteSlice;
use nom::*;

use super::parameter::{parameter, Parameter};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ArithmeticOperator {
    Sub,
    Add,
    Mul,
    Div,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    Abs(Expression),
    Acos(Expression),
    Asin(Expression),
    Atan((Expression, Expression)),
    Cos(Expression),
    Exists(Expression),
    Exp(Expression),
    Fix(Expression),
    Fup(Expression),
    Ln(Expression),
    Round(Expression),
    Sin(Expression),
    Sqrt(Expression),
    Tan(Expression),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionToken {
    ArithmeticOperator(ArithmeticOperator),
    Expression(Expression),
    Function(Function),
    Literal(f32),
    Parameter(Parameter),
}

pub type Expression = Vec<ExpressionToken>;

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

// Special snowflake ATAN with two "args"
named!(atan<CompleteByteSlice, Function>, map!(
    preceded!(
        tag_no_case!("ATAN"),
        ws!(separated_pair!(expression, char!('/'), expression))
    ),
    |(left, right)| Function::Atan((left, right))
));

named_args!(function_call<'a>(func_ident: &str)<CompleteByteSlice<'a>, Expression>,
    preceded!(tag_no_case!(func_ident), expression)
);

named!(function<CompleteByteSlice, ExpressionToken>, map!(
    alt_complete!(
        atan |
        map!(call!(function_call, "ABS"), |args| Function::Abs(args)) |
        map!(call!(function_call, "ACOS"), |args| Function::Acos(args)) |
        map!(call!(function_call, "ASIN"), |args| Function::Asin(args)) |
        map!(call!(function_call, "COS"), |args| Function::Cos(args)) |
        map!(call!(function_call, "EXP"), |args| Function::Exp(args)) |
        map!(call!(function_call, "FIX"), |args| Function::Fix(args)) |
        map!(call!(function_call, "FUP"), |args| Function::Fup(args)) |
        map!(call!(function_call, "ROUND"), |args| Function::Round(args)) |
        map!(call!(function_call, "LN"), |args| Function::Ln(args)) |
        map!(call!(function_call, "SIN"), |args| Function::Sin(args)) |
        map!(call!(function_call, "SQRT"), |args| Function::Sqrt(args)) |
        map!(call!(function_call, "TAN"), |args| Function::Tan(args)) |
        map!(call!(function_call, "EXISTS"), |args| Function::Exists(args))
    ),
    |res| ExpressionToken::Function(res)
));

named!(expression_token<CompleteByteSlice, ExpressionToken>, alt_complete!(
    function |
    literal |
    arithmetic |
    map!(parameter, |res| ExpressionToken::Parameter(res)) |
    map!(expression, |res| ExpressionToken::Expression(res))
));

named!(pub expression<CompleteByteSlice, Expression>, ws!(
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
    fn arithmetic_operators_have_the_right_precedence() {
        assert!(ArithmeticOperator::Div > ArithmeticOperator::Mul);
        assert!(ArithmeticOperator::Mul > ArithmeticOperator::Add);
        assert!(ArithmeticOperator::Add > ArithmeticOperator::Sub);
        assert!(ArithmeticOperator::Add == ArithmeticOperator::Add);
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

    #[test]
    fn it_parses_atan() {
        let input = Cbs(b"[ATAN[3 + 4]/[5]]");

        check_expression(
            expression(input),
            vec![ExpressionToken::Function(Function::Atan((
                vec![
                    ExpressionToken::Literal(3.0),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                    ExpressionToken::Literal(4.0),
                ],
                vec![ExpressionToken::Literal(5.0)],
            )))],
        );
    }

    #[test]
    fn it_parses_a_function() {
        let input = Cbs(b"[ABS[1.0]]");

        check_expression(
            expression(input),
            vec![ExpressionToken::Function(Function::Abs(vec![
                ExpressionToken::Literal(1.0),
            ]))],
        );
    }

    #[test]
    fn it_parses_functions() {
        let inputs: Vec<String> = vec![
            "[ABS[1.0]]".into(),
            "[ACOS[1.0]]".into(),
            "[ASIN[1.0]]".into(),
            "[COS[1.0]]".into(),
            "[EXP[1.0]]".into(),
            "[FIX[1.0]]".into(),
            "[FUP[1.0]]".into(),
            "[ROUND[1.0]]".into(),
            "[LN[1.0]]".into(),
            "[SIN[1.0]]".into(),
            "[SQRT[1.0]]".into(),
            "[TAN[1.0]]".into(),
            "[EXISTS[1.0]]".into(),
        ];

        for input in inputs.into_iter() {
            let parsed = expression(Cbs(input.as_bytes()));

            assert!(parsed.is_ok());
            assert_eq!(parsed.unwrap().0, EMPTY);
        }
    }

    #[test]
    fn it_parses_negative_numbers_as_negative_numbers() {
        let input = Cbs(b"[-10.0*-12]");

        check_expression(
            expression(input),
            vec![
                ExpressionToken::Literal(-10.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(-12.0),
            ],
        );
    }

    #[test]
    fn it_parses_expressions_with_parameters() {
        let _input = Cbs(b"[1 + #1234 * #<named_param> / #<_global_param>]");
    }

    #[test]
    fn it_parses_function_calls() {
        let _input = Cbs(b"[SIN[10]]");
    }
}
