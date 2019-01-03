use common::parsing::Span;
use expression::parser::{gcode_expression, gcode_parameter};
use expression::{Expression, Parameter};
use nom::*;

/// Assign a value to a variable
///
/// A value can be a literal or a complete expression
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    /// The parameter to assign a value to
    lhs: Parameter,

    /// The value or result of an expression to assign
    rhs: Expression,
}

named!(pub assignment<Span, Assignment>,
    map!(
        sep!(
            space0,
            separated_pair!(
                gcode_parameter,
                char!('='),
                gcode_expression
            )
        ),
        |(lhs, rhs)| Assignment { lhs, rhs }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, assert_parse_ok, empty_span, span};

    #[test]
    fn parse_summin() {}
}
