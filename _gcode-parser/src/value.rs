use nom::types::CompleteByteSlice;
use nom::*;

use super::helpers::float_no_exponent;
use expression::{parser::gcode_expression, parser::gcode_parameter, Expression, Parameter};

#[derive(Debug, PartialEq)]
pub enum Value {
    Unsigned(u32),
    Signed(i32),
    Float(f32),
    Parameter(Parameter),
    Expression(Expression),
}

impl Value {
    /// Attempt to cast the value into an f64
    ///
    /// This will fail if the variant is not numeric
    pub fn as_f64(&self) -> Result<f64, ()> {
        match self {
            Value::Unsigned(num) => Ok(*num as f64),
            Value::Signed(num) => Ok(*num as f64),
            Value::Float(num) => Ok(*num as f64),
            _ => Err(()),
        }
    }
}

impl From<f32> for Value {
    fn from(other: f32) -> Self {
        Value::Float(other)
    }
}

impl From<f64> for Value {
    fn from(other: f64) -> Self {
        Value::Float(other as f32)
    }
}

impl From<i32> for Value {
    fn from(other: i32) -> Self {
        Value::Signed(other)
    }
}

impl From<u32> for Value {
    fn from(other: u32) -> Self {
        Value::Unsigned(other)
    }
}

impl From<Parameter> for Value {
    fn from(other: Parameter) -> Self {
        Value::Parameter(other)
    }
}

impl From<Expression> for Value {
    fn from(other: Expression) -> Self {
        Value::Expression(other)
    }
}

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
    gcode_parameter,
    |param| Value::Parameter(param)
));

named!(value_expression<CompleteByteSlice, Value>, map!(
    gcode_expression,
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
    pub preceded_float_value<'a>(preceding: &str)<CompleteByteSlice<'a>, Value>, sep!(space0, preceded!(
    tag_no_case!(preceding),
    float_value
)));

#[cfg(test)]
mod tests {
    use super::*;
    use expression::{ArithmeticOperator, ExpressionToken};
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
