use crate::line::{line, Line};
use common::parsing::Span;
use expression::parser::{gcode_expression, gcode_non_global_ident};
use expression::{Expression, Parameter};
use nom::*;

#[derive(Debug, PartialEq, Clone)]
pub enum BranchType {
    /// If
    If,

    /// Else if
    ElseIf,

    /// Else
    Else,
}

/// An if/else if/else chain
#[derive(Debug, PartialEq, Clone)]
pub struct Branch<'a> {
    branch_type: BranchType,
    lines: Vec<Line<'a>>,
    condition: Option<Expression>,
}

/// An if/else if/else chain
#[derive(Debug, PartialEq, Clone)]
pub struct Condition<'a> {
    branches: Vec<Branch<'a>>,
}

named!(pub condition<Span, Condition>,
    sep!(
        space0,
        do_parse!(
            block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
            tag_no_case!("if") >>
            condition: gcode_expression >>
            line_ending >>
            lines: many0!(line) >>
            preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
            tag_no_case!("endif") >>
            ({
                let if_branch = Branch {
                    branch_type: BranchType::If,
                    condition: Some(condition),
                    lines
                };

                Condition {
                    branches: vec![if_branch]
                }
            })
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Feedrate, Token, TokenType};
    use common::{assert_parse, empty_span, span};
    use expression::{BinaryOperator, ExpressionToken, Value};

    #[test]
    fn parse_if() {
        assert_parse!(
            parser = condition;
            input = span!(b"o1 if [1 gt 0]\nf500\no1 endif");
            expected = Condition {
                branches: vec![
                    Branch {
                        branch_type: BranchType::If,
                        condition: Some(Expression::from_tokens(vec![
                            ExpressionToken::Literal(1.0),
                            ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                            ExpressionToken::Literal(0.0),
                        ])),
                        lines: vec![Line {
                            span: empty_span!(offset = 15, line = 2),
                            tokens: vec![
                                Token {
                                    span: empty_span!(offset = 15, line = 2),
                                    token: TokenType::Feedrate(Feedrate { feedrate: Value::Float(500.0) })
                                },
                            ]
                        }]
                    }
                ]
            };
            remaining = empty_span!(offset = 28, line = 3)
        );
    }
}
