//! Expression parser

#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

#[macro_use]
extern crate nom;
#[cfg(test)]
#[macro_use]
extern crate maplit;

mod evaluator;
pub mod parser;
mod value;

pub use self::evaluator::evaluate;
pub use self::value::Value;
use std::collections::HashMap;
use std::fmt;

/// List of parameters (variables) to pass in as the environment for the evaluation of an expression
pub type Context = HashMap<Parameter, f32>;

/// Arithmetic (`/`, `*`, `+`, `-`) operator
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ArithmeticOperator {
    /// Subtract
    Sub,
    /// Add
    Add,
    /// Multiply
    Mul,
    /// Divide
    Div,
}

/// Logical operator
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum LogicalOperator {
    /// Logical AND
    And,
    /// Logical OR
    Or,
    /// Logical NOT (negation)
    Not,
}

/// Builtin functions
#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    /// Absolute
    Abs(Expression),
    /// ACOS
    Acos(Expression),
    /// ASIN
    Asin(Expression),
    /// Arctan
    Atan((Expression, Expression)),
    /// Cos
    Cos(Expression),
    /// Check if parameter exists (1.0) or not (0.0)
    Exists(Parameter),
    /// Exponent
    Exp(Expression),
    /// Round down
    Floor(Expression),
    /// Round up
    Ceil(Expression),
    /// Ln
    Ln(Expression),
    /// Round to nearest integer
    Round(Expression),
    /// Sin
    Sin(Expression),
    /// Square root
    Sqrt(Expression),
    /// Tan
    Tan(Expression),
}

/// Comparison operators
#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterThanOrEqual,
    /// `<`
    LessThan,
    /// `<=`
    LessThanOrEqual,
}

/// Union of any possible token present in an expression
#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionToken {
    /// Comparison
    BinaryOperator(BinaryOperator),
    /// General arithmetic
    ArithmeticOperator(ArithmeticOperator),
    /// Logical operator
    LogicalOperator(LogicalOperator),
    /// Nested expressions
    Expression(Expression),
    /// Builtin function
    Function(Function),
    /// Number, always parsed to float
    Literal(f32),
    /// Parameter
    Parameter(Parameter),
}

/// Wrapping expression type
#[derive(Clone, Debug, PartialEq)]
pub struct Expression(pub Vec<ExpressionToken>);

impl From<Vec<ExpressionToken>> for Expression {
    fn from(other: Vec<ExpressionToken>) -> Self {
        Expression(other)
    }
}

/// A parameter (variable)
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub enum Parameter {
    /// Numbered parameter like `#4711`
    Numbered(u32),
    /// Named local parameter like `#<some_local_param>`
    Named(String),
    /// Named global parameter like `#<_some_global_param>`
    Global(String),
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Parameter::Numbered(n) => write!(f, "{}", n),
            Parameter::Named(name) => write!(f, "<{}>", name),
            Parameter::Global(name) => write!(f, "<_{}>", name),
        }
    }
}
