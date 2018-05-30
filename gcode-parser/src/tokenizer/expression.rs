use nom::types::CompleteByteSlice;
use nom::*;

#[derive(Debug, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum Function {
    Atan((Expression, Expression)),
}

#[derive(Debug, PartialEq)]
pub enum ExpressionToken {
    Expression(Expression),
    Literal(f32),
    ArithmeticOperator(ArithmeticOperator),
    Function(Function),
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
        tag_no_case_s!("ATAN"),
        ws!(separated_pair!(expression, char!('/'), expression))
    ),
    |(left, right)| Function::Atan((left, right))
));

// named_args!(function_call<'a>(func_ident: &str)<CompleteByteSlice<'a>, Function>, map!(
//     tuple!(tag_no_case!(func_ident), expression),
//     |(name, arg)| {

//         Function::Atan((ExpressionToken::Literal(0.0), ExpressionToken::Literal(0.0)))
//     }
// ));

// named!(function<CompleteByteSlice, ExpressionToken>, map!(
//     tuple!(
//         alt_complete!(
//             call!("", atan_args) |
//             tag_no_case_s!("ABS") |
//             tag_no_case_s!("ACOS") |
//             tag_no_case_s!("ASIN") |
//             tag_no_case_s!("COS") |
//             tag_no_case_s!("EXP") |
//             tag_no_case_s!("FIX") |
//             tag_no_case_s!("FUP") |
//             tag_no_case_s!("ROUND") |
//             tag_no_case_s!("LN") |
//             tag_no_case_s!("SIN") |
//             tag_no_case_s!("SQRT") |
//             tag_no_case_s!("TAN") |
//             tag_no_case_s!("EXISTS")
//         ),
//         expression
//     ),
//     |(function, arg)| {
//         println!("{:?}, {:?}", function, arg);
//         // match function.to_lowercase() {

//         // }
//         ExpressionToken::Literal(0.0)
//     }
// ));

named!(expression_token<CompleteByteSlice, ExpressionToken>, alt_complete!(
    map!(atan, |res| ExpressionToken::Function(res)) |
    literal |
    arithmetic |
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
    fn it_parses_expressions_with_parameters() {
        let _input = Cbs(b"[1 + #1234 * #<named_param> / #<_global_param>]");
    }

    #[test]
    fn it_parses_function_calls() {
        let _input = Cbs(b"[SIN[10]]");
    }
}
