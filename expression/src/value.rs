use crate::Expression;
use crate::Parameter;

/// A value
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    /// Unsigned integer
    Unsigned(u32),
    /// Signed integer
    Signed(i32),
    /// Single precision floating point number
    Float(f32),
    /// A parameter or variable substitution
    Parameter(Parameter),
    /// An expression that resolves to a literal value
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
