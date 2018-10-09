use crate::Expression;
use crate::Parameter;

#[derive(Debug, PartialEq)]
pub enum Value {
    Unsigned(u32),
    Signed(i32),
    Float(f32),
    Parameter(Parameter),
    Expression(Expression),
}

impl From<f32> for Value {
    fn from(other: f32) -> Self {
        Value::Float(other)
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
