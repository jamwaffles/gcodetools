//! Expression parser

#![deny(
    // TODO: Turn back on
    // missing_docs,
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

/// Arithmetic (`/`, `*`, `+`, `-` and modulo) operator
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ArithmeticOperator {
    /// Subtract
    Sub,
    /// Add
    Add,
    /// Multiply
    Mul,
    /// Modulo
    Mod,
    /// Divide
    Div,
}

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArithmeticOperator::Sub => write!(f, "-"),
            ArithmeticOperator::Add => write!(f, "+"),
            ArithmeticOperator::Mul => write!(f, "*"),
            ArithmeticOperator::Mod => write!(f, "mod"),
            ArithmeticOperator::Div => write!(f, "/"),
        }
    }
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

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogicalOperator::And => write!(f, "AND"),
            LogicalOperator::Or => write!(f, "OR"),
            LogicalOperator::Not => write!(f, "NOT"),
        }
    }
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

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Abs(expr) => write!(f, "abs{}", expr),
            Function::Acos(expr) => write!(f, "acos{}", expr),
            Function::Asin(expr) => write!(f, "asin{}", expr),
            Function::Atan((expr1, expr2)) => write!(f, "atan[{},{}]", expr1, expr2),
            Function::Cos(expr) => write!(f, "cos{}", expr),
            Function::Exists(param) => write!(f, "exists{}", param),
            Function::Exp(expr) => write!(f, "exp{}", expr),
            Function::Floor(expr) => write!(f, "floor{}", expr),
            Function::Ceil(expr) => write!(f, "ceil{}", expr),
            Function::Ln(expr) => write!(f, "ln{}", expr),
            Function::Round(expr) => write!(f, "round{}", expr),
            Function::Sin(expr) => write!(f, "sin{}", expr),
            Function::Sqrt(expr) => write!(f, "sqrt{}", expr),
            Function::Tan(expr) => write!(f, "tan{}", expr),
        }
    }
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

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOperator::Equal => write!(f, "EQ"),
            BinaryOperator::NotEqual => write!(f, "NE"),
            BinaryOperator::GreaterThan => write!(f, "GT"),
            BinaryOperator::GreaterThanOrEqual => write!(f, "GE"),
            BinaryOperator::LessThan => write!(f, "LT"),
            BinaryOperator::LessThanOrEqual => write!(f, "LE"),
        }
    }
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

impl fmt::Display for ExpressionToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpressionToken::BinaryOperator(item) => write!(f, "{}", item),
            ExpressionToken::ArithmeticOperator(item) => write!(f, "{}", item),
            ExpressionToken::LogicalOperator(item) => write!(f, "{}", item),
            ExpressionToken::Expression(item) => write!(f, "{}", item),
            ExpressionToken::Function(item) => write!(f, "{}", item),
            ExpressionToken::Literal(item) => write!(f, "{}", item),
            ExpressionToken::Parameter(item) => write!(f, "{}", item),
        }
    }
}

/// Wrapping expression type
#[derive(Clone, Debug, PartialEq)]
pub struct Expression(pub Vec<ExpressionToken>);

impl From<Vec<ExpressionToken>> for Expression {
    fn from(other: Vec<ExpressionToken>) -> Self {
        Expression(other)
    }
}

impl Expression {
    /// Create an expression from a list of tokens
    pub fn from_tokens(tokens: Vec<ExpressionToken>) -> Self {
        tokens.into()
    }

    /// Create an empty expression
    pub fn empty() -> Self {
        Self(Vec::new())
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tokens = self
            .0
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "[{}]", tokens)
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

impl Parameter {
    /// Convert this parameter into its string identifier representation
    ///
    /// To output the parameter with a leading `#`, use `format!()` to call the `Display` impl for
    /// this struct.
    ///
    /// ```
    /// use expression::Parameter;
    ///
    /// assert_eq!(Parameter::Numbered(101).to_ident_string(), "101");
    /// assert_eq!(Parameter::Named("some_name".to_string()).to_ident_string(), "<some_name>");
    /// assert_eq!(Parameter::Global("some_global".to_string()).to_ident_string(), "<_some_global>");
    /// ```
    pub fn to_ident_string(&self) -> String {
        match self {
            Parameter::Numbered(n) => n.to_string(),
            Parameter::Named(name) => format!("<{}>", name),
            Parameter::Global(global) => format!("<_{}>", global),
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.to_ident_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format() {
        assert_eq!(
            Expression::from_tokens(vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Literal(2.0),
            ])
            .to_string(),
            "[1 + 2]"
        );

        assert_eq!(
            Expression::from_tokens(vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Expression(
                    vec![
                        ExpressionToken::Literal(2.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Literal(3.0),
                    ]
                    .into(),
                ),
            ])
            .to_string(),
            "[1 + [2 + 3]]"
        );

        assert_eq!(
            Expression::from_tokens(vec![
                ExpressionToken::Parameter(Parameter::Numbered(1234)),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Expression(
                    vec![
                        ExpressionToken::Parameter(Parameter::Named("named".into())),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Parameter(Parameter::Global("global".into())),
                    ]
                    .into(),
                ),
            ])
            .to_string(),
            "[#1234 + [#<named> + #<_global>]]"
        );
    }
}
