use super::helpers::float_no_exponent;
use super::parameter::{not_numbered_parameter, parameter};
use crate::{
    ArithmeticOperator, BinaryOperator, Expression, ExpressionToken, Function, LogicalOperator,
    Parameter,
};
use common::parsing::Span;
use nom::*;

named!(literal<Span, ExpressionToken>, map!(
    float_no_exponent,
    |res| ExpressionToken::Literal(res)
));

named!(arithmetic<Span, ExpressionToken>,
    map!(
        alt!(
            map!(char!('+'), |_| ArithmeticOperator::Add) |
            map!(char!('-'), |_| ArithmeticOperator::Sub) |
            map!(char!('*'), |_| ArithmeticOperator::Mul) |
            map!(char!('/'), |_| ArithmeticOperator::Div) |
            map!(tag_no_case!("mod"), |_| ArithmeticOperator::Mod)
        ),
        |res| ExpressionToken::ArithmeticOperator(res)
    )
);

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
        sep!(space0, separated_pair!(expression, char!('/'), expression))
    ),
    |(left, right)| Function::Atan((left, right))
));

// Exists is a function, but only allows named/global params as args
named!(exists<Span, Parameter>, preceded!(
    tag_no_case!("EXISTS"),
    sep!(space0, delimited!(char!('['), not_numbered_parameter, char!(']')))
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
    pub expression<Span, Expression>,
    map!(
        sep!(
            space0,
            alt!(
                delimited!(
                    char!('['),
                    sep!(space0, many1!(expression_token)),
                    char!(']')
                ) |
                map!(function, |f| vec![ f ])
            )
        ),
        |tokens| Expression(tokens)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BinaryOperator;
    use common::{assert_parse, assert_parse_ok, span};

    #[test]
    fn arithmetic_operators_have_the_right_precedence() {
        assert!(ArithmeticOperator::Div > ArithmeticOperator::Mul);
        assert!(ArithmeticOperator::Mul > ArithmeticOperator::Add);
        assert!(ArithmeticOperator::Add > ArithmeticOperator::Sub);
        assert!(ArithmeticOperator::Add == ArithmeticOperator::Add);
    }

    #[test]
    fn it_parses_simple_expressions() {
        assert_parse!(
            parser = expression;
            input = span!(b"[1]");
            expected = vec![ExpressionToken::Literal(1.0)].into()
        );
    }

    #[test]
    fn modulo() {
        assert_parse!(
            parser = expression;
            input = span!(b"[10 mod 3]");
            expected = vec![
                ExpressionToken::Literal(10.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mod),
                ExpressionToken::Literal(3.0),
            ].into()
        );
    }

    #[test]
    fn it_parses_arithmetic() {
        assert_parse!(
            parser = expression;
            input = span!(b"[1 + 2 * 3 / 4 - 5]");
            expected = vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Literal(2.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(3.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Div),
                ExpressionToken::Literal(4.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
                ExpressionToken::Literal(5.0),
            ].into()
        );
    }

    #[test]
    fn whitespace() {
        assert_parse!(
            parser = expression;
            input = span!(b"[ 1 + 2 * 3 / 4 - 5 ]");
            expected = vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Literal(2.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(3.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Div),
                ExpressionToken::Literal(4.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
                ExpressionToken::Literal(5.0),
            ].into()
        );
    }

    #[test]
    fn it_parses_nested_expressions() {
        assert_parse!(
            parser = expression;
            input = span!(b"[1 + [[2 - 3] * 4]]");
            expected = vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Expression(vec![
                    ExpressionToken::Expression(vec![
                        ExpressionToken::Literal(2.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
                        ExpressionToken::Literal(3.0),
                    ].into()),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                    ExpressionToken::Literal(4.0),
                ].into()),
            ].into()
        );
    }

    #[test]
    fn it_parses_atan() {
        assert_parse!(
            parser = expression;
            input = span!(b"[ATAN[3 + 4]/[5]]");
            expected =
                vec![ExpressionToken::Function(Function::Atan((
                    vec![
                        ExpressionToken::Literal(3.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Literal(4.0),
                    ].into(),
                    vec![ExpressionToken::Literal(5.0)].into(),
                )))].into();
        );
    }

    #[test]
    fn it_parses_a_function() {
        assert_parse!(
            parser = expression;
            input =
                span!(b"[ABS[1.0]]"),
                span!(b"ABS[1.0]")
            ;
            expected =
                vec![ExpressionToken::Function(Function::Abs(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Abs(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into()
            ;
        );
    }

    #[test]
    fn it_parses_functions() {
        assert_parse!(
            parser = expression;
            input =
                span!(b"[ABS[1.0]]"),
                span!(b"[ACOS[1.0]]"),
                span!(b"[ASIN[1.0]]"),
                span!(b"[COS[1.0]]"),
                span!(b"[EXP[1.0]]"),
                span!(b"[FIX[1.0]]"),
                span!(b"[FUP[1.0]]"),
                span!(b"[ROUND[1.0]]"),
                span!(b"[LN[1.0]]"),
                span!(b"[SIN[1.0]]"),
                span!(b"[SQRT[1.0]]"),
                span!(b"[TAN[1.0]]"),
                span!(b"[EXISTS[#<named>]]")
            ;

            expected =
                vec![ExpressionToken::Function(Function::Abs(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Acos(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Asin(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Cos(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Exp(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Floor(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Ceil(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Round(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Ln(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Sin(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Sqrt(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Tan(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Exists(Parameter::Named("named".into())))].into()
            ;
        );
    }

    #[test]
    fn it_parses_binary_operators() {
        assert_parse!(
            parser = expression;
            input =
                span!(b"[1 EQ 2]"),
                span!(b"[1 NE 2]"),
                span!(b"[1 GT 2]"),
                span!(b"[1 GE 2]"),
                span!(b"[1 LT 2]"),
                span!(b"[1 LE 2]")
            ;
            expected =
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::Equal), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::NotEqual), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::GreaterThanOrEqual), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::LessThan), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::LessThanOrEqual), ExpressionToken::Literal(2.0)].into()
            ;
        );
    }

    #[test]
    fn it_parses_logical_operators() {
        assert_parse!(
            parser = expression;
            input =
                span!(b"[1 AND 2]"),
                span!(b"[1 OR 2]"),
                span!(b"[1 NOT 2]"),
                span!(b"[[#<fraction> GT .99] OR [#<fraction> LT .01]]")
            ;
            expected =
                vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::LogicalOperator(LogicalOperator::And),
                    ExpressionToken::Literal(2.0),
                ].into(),
                vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::LogicalOperator(LogicalOperator::Or),
                    ExpressionToken::Literal(2.0),
                ].into(),
                vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::LogicalOperator(LogicalOperator::Not),
                    ExpressionToken::Literal(2.0),
                ].into(),
                vec![
                    ExpressionToken::Expression(vec![
                        ExpressionToken::Parameter(Parameter::Named("fraction".into())),
                        ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                        ExpressionToken::Literal(0.99),
                    ].into()),
                    ExpressionToken::LogicalOperator(LogicalOperator::Or),
                    ExpressionToken::Expression(vec![
                        ExpressionToken::Parameter(Parameter::Named("fraction".into())),
                        ExpressionToken::BinaryOperator(BinaryOperator::LessThan),
                        ExpressionToken::Literal(0.01),
                    ].into()),
                ].into()
            ;
        );
    }

    #[test]
    fn it_parses_negative_numbers_as_negative_numbers() {
        assert_parse!(
            parser = expression;
            input = span!(b"[-10.0*-12]");
            expected = vec![
                ExpressionToken::Literal(-10.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(-12.0),
            ].into();
        );
    }

    #[test]
    fn it_parses_expressions_with_parameters() {
        assert_parse_ok!(
            parser = expression,
            input = span!(b"[1 + #1234 * #<named_param> / #<_global_param>]")
        );
    }

    #[test]
    fn it_parses_function_calls() {
        assert_parse_ok!(parser = expression, input = span!(b"[SIN[10]]"));
    }

    #[test]
    fn it_parses_exists_calls() {
        assert_parse!(
            parser = expression;
            input = span!(b"[EXISTS[#<named_param>]]");
            expected = vec![ExpressionToken::Function(Function::Exists(
                Parameter::Named("named_param".into()),
            ))].into();
        );
    }
}
