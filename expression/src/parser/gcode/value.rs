// TODO: Merge with ngc_float in gcode-parser

use super::expression::expression;
use super::helpers::float_no_exponent;
use super::parameter::parameter;
use crate::value::Value;
use gcode_parser::Span;
use nom::digit;

named!(value_signed<Span, Value>, map!(
    flat_map!(
        recognize!(preceded!(opt!(one_of!("+-")), digit)),
        parse_to!(i32)
    ),
    |res| Value::Signed(res)
));

named!(value_unsigned<Span, Value>, map!(
    flat_map!(
        recognize!(digit),
        parse_to!(u32)
    ),
    |res| Value::Unsigned(res)
));

named!(value_float<Span, Value>, map!(
    float_no_exponent,
    |res| Value::Float(res)
));

named!(value_parameter<Span, Value>, map!(
    parameter,
    |param| Value::Parameter(param)
));

named!(value_expression<Span, Value>, map!(
    expression,
    |expr| Value::Expression(expr)
));

named!(
    pub unsigned_value<Span, Value>, alt_complete!(
    value_unsigned |
    value_parameter |
    value_expression
));

named_args!(
    pub preceded_unsigned_value<'a>(preceding: &str)<Span<'a>, Value>, preceded!(
    tag_no_case!(preceding),
    unsigned_value
));

named_args!(
    pub preceded_signed_value<'a>(preceding: &str)<Span<'a>, Value>, preceded!(
    tag_no_case!(preceding),
    alt_complete!(
        value_signed |
        value_parameter |
        value_expression
    )
));

named!(
    pub float_value<Span, Value>, alt_complete!(
    value_float |
    value_parameter |
    value_expression
));

named_args!(
    pub preceded_float_value<'a>(preceding: &str)<Span<'a>, Value>, ws!(preceded!(
    tag_no_case!(preceding),
    float_value
)));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{span, ArithmeticOperator, ExpressionToken, Parameter};

    #[test]
    fn it_parses_signed_integers_or_else() {
        assert_eq!(
            preceded_signed_value(span!(b"A10"), "A").unwrap().1,
            Value::Signed(10i32)
        );

        assert_eq!(
            preceded_signed_value(span!(b"A-10"), "A").unwrap().1,
            Value::Signed(-10i32)
        );

        assert_eq!(
            preceded_signed_value(span!(b"A#<test>"), "A").unwrap().1,
            Value::Parameter(Parameter::Named("test".into()))
        );

        assert_eq!(
            preceded_signed_value(span!(b"A[1 + 2]"), "A").unwrap().1,
            Value::Expression(vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Literal(2.0),
            ])
        );
    }

    #[test]
    fn it_parses_preceded_expressions() {
        assert_eq!(
            preceded_float_value(span!(b"Z[#<zscale>*10.]"), "Z")
                .unwrap()
                .1,
            Value::Expression(vec![
                ExpressionToken::Parameter(Parameter::Named("zscale".into())),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(10.0),
            ])
        );
    }
}
