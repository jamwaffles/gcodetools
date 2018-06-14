pub mod evaluator;
pub mod parser;

pub use self::parser::expression;
use super::prelude::Parameter;

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
pub type Expression = Vec<ExpressionToken>;
