use crate::{Expression, Parameter};
use std::fmt;

/// A value
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    /// Unsigned integer
    Unsigned(u64),
    /// Signed integer
    Signed(i64),
    /// Double precision floating point number
    Float(f64),
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
            Value::Float(n) => n,
            _ => panic!("Attempted to convert non-numeric value to f64"),
        }
    }
}

macro_rules! impl_from {
    ($ty_in:ty => $ty_out:path) => {
        impl From<$ty_in> for Value {
            fn from(other: $ty_in) -> Self {
                $ty_out(other.into())
            }
        }
    };
}

impl_from!(u32 => Value::Unsigned);
impl_from!(i32 => Value::Signed);
impl_from!(f32 => Value::Float);
impl_from!(u64 => Value::Unsigned);
impl_from!(i64 => Value::Signed);
impl_from!(f64 => Value::Float);
impl_from!(Parameter => Value::Parameter);
impl_from!(Expression => Value::Expression);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Parameter(param) => write!(f, "{}", param),
            Value::Expression(expr) => write!(f, "{}", expr),
            Value::Unsigned(n) => write!(f, "{}", n),
            Value::Signed(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
        }
    }
}
