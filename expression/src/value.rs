use crate::{Expression, Parameter};
use std::fmt;

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

impl Value {
    /// Convert a numeric value to an f64. Will panic if value is not an integer of f32
    pub fn as_f64_unchecked(&self) -> f64 {
        match *self {
            Value::Unsigned(n) => n as f64,
            Value::Signed(n) => n as f64,
            Value::Float(n) => n as f64,
            _ => panic!("Attempted to convert non-numeric value to f64"),
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Parameter(param) => write!(f, "{}", param),
            Value::Expression(expr) => write!(f, "{}", expr),
            Value::Unsigned(n) => write!(f, "{}", n),
            Value::Signed(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
        }
    }
}
