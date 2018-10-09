use super::expression::expression;
use super::helpers::float_no_exponent;
use super::parameter::parameter;
use crate::value::Value;
use nom::digit;
use nom::types::CompleteByteSlice;

named!(value_signed<CompleteByteSlice, Value>, map!(
    flat_map!(
        recognize!(preceded!(opt!(one_of!("+-")), digit)),
        parse_to!(i32)
    ),
    |res| Value::Signed(res)
));

named!(value_unsigned<CompleteByteSlice, Value>, map!(
    flat_map!(
        recognize!(digit),
        parse_to!(u32)
    ),
    |res| Value::Unsigned(res)
));

named!(value_float<CompleteByteSlice, Value>, map!(
    float_no_exponent,
    |res| Value::Float(res)
));

named!(value_parameter<CompleteByteSlice, Value>, map!(
    parameter,
    |param| Value::Parameter(param)
));

named!(value_expression<CompleteByteSlice, Value>, map!(
    expression,
    |expr| Value::Expression(expr)
));

named!(
    pub unsigned_value<CompleteByteSlice, Value>, alt_complete!(
    value_unsigned |
    value_parameter |
    value_expression
));

named_args!(
    pub preceded_unsigned_value<'a>(preceding: &str)<CompleteByteSlice<'a>, Value>, preceded!(
    tag_no_case!(preceding),
    unsigned_value
));

named_args!(
    pub preceded_signed_value<'a>(preceding: &str)<CompleteByteSlice<'a>, Value>, preceded!(
    tag_no_case!(preceding),
    alt_complete!(
        value_signed |
        value_parameter |
        value_expression
    )
));

named!(
    pub float_value<CompleteByteSlice, Value>, alt_complete!(
    value_float |
    value_parameter |
    value_expression
));

named_args!(
    pub preceded_float_value<'a>(preceding: &str)<CompleteByteSlice<'a>, Value>, ws!(preceded!(
    tag_no_case!(preceding),
    float_value
)));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArithmeticOperator, ExpressionToken, Parameter};
    use nom::types::CompleteByteSlice as Cbs;

    #[test]
    fn it_parses_signed_integers_or_else() {
        assert_complete_parse!(
            preceded_signed_value(Cbs(b"A10"), "A"),
            Value::Signed(10i32)
        );

        assert_complete_parse!(
            preceded_signed_value(Cbs(b"A-10"), "A"),
            Value::Signed(-10i32)
        );

        assert_complete_parse!(
            preceded_signed_value(Cbs(b"A#<test>"), "A"),
            Value::Parameter(Parameter::Named("test".into()))
        );

        assert_complete_parse!(
            preceded_signed_value(Cbs(b"A[1 + 2]"), "A"),
            Value::Expression(vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Literal(2.0),
            ])
        );
    }

    #[test]
    fn it_parses_preceded_expressions() {
        assert_complete_parse!(
            preceded_float_value(Cbs(b"Z[#<zscale>*10.]"), "Z"),
            Value::Expression(vec![
                ExpressionToken::Parameter(Parameter::Named("zscale".into())),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(10.0),
            ])
        );
    }
}
