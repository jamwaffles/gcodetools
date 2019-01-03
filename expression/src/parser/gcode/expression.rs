use super::helpers::float_no_exponent;
use super::parameter::{not_numbered_parameter, parameter};
use crate::{
    ArithmeticOperator, BinaryOperator, Expression, ExpressionToken, Function, LogicalOperator,
    Parameter,
};
use gcode_parser::Span;

named!(literal<Span, ExpressionToken>, map!(
    float_no_exponent,
    |res| ExpressionToken::Literal(res)
));

named!(arithmetic<Span, ExpressionToken>, map!(
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

named!(logical_operator<Span, ExpressionToken>, map!(
    alt_complete!(
        map!(tag_no_case!("AND"), |_| LogicalOperator::And) |
        map!(tag_no_case!("OR"), |_| LogicalOperator::Or) |
        map!(tag_no_case!("NOT"), |_| LogicalOperator::Not)
    ),
    |res| ExpressionToken::LogicalOperator(res)
));

// Special snowflake ATAN with two "args"
named!(atan<Span, Function>, map!(
    preceded!(
        tag_no_case!("ATAN"),
        ws!(separated_pair!(expression, char!('/'), expression))
    ),
    |(left, right)| Function::Atan((left, right))
));

// Exists is a function, but only allows named/global params as args
named!(exists<Span, Parameter>, preceded!(
    tag_no_case!("EXISTS"),
    ws!(delimited!(char!('['), not_numbered_parameter, char!(']')))
));

named_args!(function_call<'a>(func_ident: &str)<Span<'a>, Expression>,
    preceded!(tag_no_case!(func_ident), expression)
);

named!(function<Span, ExpressionToken>, map!(
    alt_complete!(
        atan |
        map!(exists, |param| Function::Exists(param)) |
        map!(call!(function_call, "ABS"), |args| Function::Abs(args)) |
        map!(call!(function_call, "ACOS"), |args| Function::Acos(args)) |
        map!(call!(function_call, "ASIN"), |args| Function::Asin(args)) |
        map!(call!(function_call, "COS"), |args| Function::Cos(args)) |
        map!(call!(function_call, "EXP"), |args| Function::Exp(args)) |
        map!(call!(function_call, "FIX"), |args| Function::Floor(args)) |
        map!(call!(function_call, "FUP"), |args| Function::Ceil(args)) |
        map!(call!(function_call, "ROUND"), |args| Function::Round(args)) |
        map!(call!(function_call, "LN"), |args| Function::Ln(args)) |
        map!(call!(function_call, "SIN"), |args| Function::Sin(args)) |
        map!(call!(function_call, "SQRT"), |args| Function::Sqrt(args)) |
        map!(call!(function_call, "TAN"), |args| Function::Tan(args))
    ),
    |res| ExpressionToken::Function(res)
));

named!(comparison<Span, ExpressionToken>, map!(
    alt_complete!(
        map!(tag_no_case!("EQ"), |_| BinaryOperator::Equal) |
        map!(tag_no_case!("NE"), |_| BinaryOperator::NotEqual) |
        map!(tag_no_case!("GT"), |_| BinaryOperator::GreaterThan) |
        map!(tag_no_case!("GE"), |_| BinaryOperator::GreaterThanOrEqual) |
        map!(tag_no_case!("LT"), |_| BinaryOperator::LessThan) |
        map!(tag_no_case!("LE"), |_| BinaryOperator::LessThanOrEqual)
    ),
    |res| ExpressionToken::BinaryOperator(res)
));

named!(expression_token<Span, ExpressionToken>, alt_complete!(
    function |
    literal |
    arithmetic |
    logical_operator |
    comparison |
    map!(parameter, |res| ExpressionToken::Parameter(res)) |
    map!(expression, |res| ExpressionToken::Expression(res))
));

named_attr!(
    #[doc = "Parse an expression"],
    pub expression<Span, Expression>, ws!(
        delimited!(
            char!('['),
            many1!(expression_token),
            char!(']')
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{empty_span, span};

    macro_rules! assert_expr {
        ($to_check:expr, $against:expr) => {
            assert_eq!($to_check, Ok((empty_span!(), $against)))
        };
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
        let input = span!(b"[1]");

        assert_expr!(expression(input), vec![ExpressionToken::Literal(1.0)]);
    }

    #[test]
    fn it_parses_arithmetic() {
        let input = span!(b"[1 + 2 * 3 / 4 - 5]");

        assert_expr!(
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
            ]
        );
    }

    #[test]
    fn it_parses_nested_expressions() {
        let input = span!(b"[1 + [[2 - 3] * 4]]");

        assert_expr!(
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
            ]
        );
    }

    #[test]
    fn it_parses_atan() {
        let input = span!(b"[ATAN[3 + 4]/[5]]");

        assert_expr!(
            expression(input),
            vec![ExpressionToken::Function(Function::Atan((
                vec![
                    ExpressionToken::Literal(3.0),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                    ExpressionToken::Literal(4.0),
                ],
                vec![ExpressionToken::Literal(5.0)],
            )))]
        );
    }

    #[test]
    fn it_parses_a_function() {
        let input = span!(b"[ABS[1.0]]");

        assert_expr!(
            expression(input),
            vec![ExpressionToken::Function(Function::Abs(vec![
                ExpressionToken::Literal(1.0),
            ]))]
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
            "[EXISTS[#<named>]]".into(),
        ];

        for input in inputs.into_iter() {
            let parsed = expression(span!(input.as_bytes()));

            assert!(parsed.is_ok());
            assert_eq!(parsed.unwrap().0, empty_span!());
        }
    }

    #[test]
    fn it_parses_binary_operators() {
        let inputs: Vec<String> = vec![
            "[1 EQ 2]".into(),
            "[1 NE 2]".into(),
            "[1 GT 2]".into(),
            "[1 GE 2]".into(),
            "[1 LT 2]".into(),
            "[1 LE 2]".into(),
        ];

        for input in inputs.into_iter() {
            let parsed = expression(span!(input.as_bytes()));

            assert!(parsed.is_ok());
            assert_eq!(parsed.unwrap().0, empty_span!());
        }
    }

    #[test]
    fn it_parses_logical_operators() {
        let inputs: Vec<(String, Expression)> = vec![
            (
                "[1 AND 2]".into(),
                vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::LogicalOperator(LogicalOperator::And),
                    ExpressionToken::Literal(2.0),
                ],
            ),
            (
                "[1 OR 2]".into(),
                vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::LogicalOperator(LogicalOperator::Or),
                    ExpressionToken::Literal(2.0),
                ],
            ),
            (
                "[1 NOT 2]".into(),
                vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::LogicalOperator(LogicalOperator::Not),
                    ExpressionToken::Literal(2.0),
                ],
            ),
            (
                "[[#<fraction> GT .99] OR [#<fraction> LT .01]]".into(),
                vec![
                    ExpressionToken::Expression(vec![
                        ExpressionToken::Parameter(Parameter::Named("fraction".into())),
                        ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                        ExpressionToken::Literal(0.99),
                    ]),
                    ExpressionToken::LogicalOperator(LogicalOperator::Or),
                    ExpressionToken::Expression(vec![
                        ExpressionToken::Parameter(Parameter::Named("fraction".into())),
                        ExpressionToken::BinaryOperator(BinaryOperator::LessThan),
                        ExpressionToken::Literal(0.01),
                    ]),
                ],
            ),
        ];

        for (input, expected) in inputs.into_iter() {
            let parsed = expression(span!(input.as_bytes()))
                .expect(&format!("Could not parse expr {}", input));

            assert_eq!(parsed.0, empty_span!());
            assert_eq!(parsed.1, expected);
        }
    }

    #[test]
    fn it_parses_negative_numbers_as_negative_numbers() {
        let input = span!(b"[-10.0*-12]");

        assert_expr!(
            expression(input),
            vec![
                ExpressionToken::Literal(-10.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(-12.0),
            ]
        );
    }

    #[test]
    fn it_parses_expressions_with_parameters() {
        let _input = span!(b"[1 + #1234 * #<named_param> / #<_global_param>]");
    }

    #[test]
    fn it_parses_function_calls() {
        let _input = span!(b"[SIN[10]]");
    }

    #[test]
    fn it_parses_exists_calls() {
        assert_expr!(
            expression(span!(b"[EXISTS[#<named_param>]]")),
            vec![ExpressionToken::Function(Function::Exists(
                Parameter::Named("named_param".into()),
            ))]
        );
    }
}
