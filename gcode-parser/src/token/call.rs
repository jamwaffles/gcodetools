use common::parsing::Span;
use expression::parser::{gcode_expression, gcode_non_global_ident};
use expression::{Expression, Parameter};
use nom::*;

/// Which type of block this is
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    subroutine_ident: Parameter,
    arguments: Vec<Expression>,
}

named!(pub call<Span, Call>,
    sep!(
        space0,
        do_parse!(
            subroutine_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
            tag_no_case!("call") >>
            arguments: many0!(gcode_expression) >>
            ({
                Call {
                    subroutine_ident,
                    arguments
                }
            })
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};
    use expression::{ArithmeticOperator, Expression, ExpressionToken, Parameter};

    #[test]
    fn parse_call() {
        assert_parse!(
            parser = call;
            input = span!(r#"o100 call [100] [1 + 2]"#
                .as_bytes());
            expected = Call {
                subroutine_ident: Parameter::Numbered(100),
                arguments: vec![
                    Expression::from_tokens(vec![
                        ExpressionToken::Literal(100.0)
                    ]),
                    Expression::from_tokens(vec![
                        ExpressionToken::Literal(1.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Literal(2.0),
                    ])
                ]
            };
        );
    }
}
