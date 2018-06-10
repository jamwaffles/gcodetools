pub mod evaluator;
pub mod parser;

pub use self::parser::expression;
use super::tokenizer::prelude::Parameter;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ArithmeticOperator {
    Sub,
    Add,
    Mul,
    Div,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    Abs(Expression),
    Acos(Expression),
    Asin(Expression),
    Atan((Expression, Expression)),
    Cos(Expression),
    Exists(Parameter),
    Exp(Expression),
    Floor(Expression),
    Ceil(Expression),
    Ln(Expression),
    Round(Expression),
    Sin(Expression),
    Sqrt(Expression),
    Tan(Expression),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionToken {
    BinaryOperator(BinaryOperator),
    ArithmeticOperator(ArithmeticOperator),
    LogicalOperator(LogicalOperator),
    Expression(Expression),
    Function(Function),
    Literal(f32),
    Parameter(Parameter),
}

pub type Expression = Vec<ExpressionToken>;
