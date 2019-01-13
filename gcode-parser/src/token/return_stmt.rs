use common::parsing::Span;
use expression::parser::{gcode_expression, gcode_non_global_ident};
use expression::{Expression, Parameter};
use nom::*;

/// Which type of block this is
#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    ident: Parameter,
    value: Option<Expression>,
}

named!(pub return_stmt<Span, Return>,
    sep!(
        space0,
        do_parse!(
            ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
            tag_no_case!("return") >>
            value: opt!(gcode_expression) >>
            ({
                Return {
                    ident,
                    value
                }
            })
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};
    use expression::{ArithmeticOperator, Expression, ExpressionToken};

    #[test]
    fn parse_return() {
        assert_parse!(
            parser = return_stmt;
            input = span!(b"o100 return [1 + 2]");
            expected = Return {
                ident: Parameter::Numbered(100),
                value: Some(Expression::from_tokens(vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                    ExpressionToken::Literal(2.0),
                ]))
            };
        );
    }

    #[test]
    fn parse_return_no_value() {
        assert_parse!(
            parser = return_stmt;
            input = span!(b"o100 return");
            expected = Return {
                ident: Parameter::Numbered(100),
                value: None
            };
        );
    }
}
