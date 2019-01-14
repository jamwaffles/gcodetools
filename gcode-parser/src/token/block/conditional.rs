use crate::line::{line, Line};
use crate::token::{comment, Comment};
use common::parsing::Span;
use expression::parser::{gcode_expression, gcode_non_global_ident};
use expression::{Expression, Parameter};
use nom::*;

/// What type of branch this is
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
    trailing_comment: Option<Comment>,
}

/// An if/else if/else chain
#[derive(Debug, PartialEq, Clone)]
pub struct Conditional<'a> {
    identifier: Parameter,
    branches: Vec<Branch<'a>>,
}

named_args!(elseif(ident: String)<Span, Branch>,
    sep!(
        space0,
        do_parse!(
            preceded!(char_no_case!('O'), tag_no_case!(ident.as_str())) >>
            tag_no_case!("elseif") >>
            condition: gcode_expression >>
            trailing_comment: opt!(comment) >>
            line_ending >>
            lines: many0!(line) >>
            ({
                Branch {
                    branch_type: BranchType::ElseIf,
                    condition: Some(condition),
                    lines,
                    trailing_comment,
                }
            })
        )
    )
);

named_args!(else_block(ident: String)<Span, Branch>,
    sep!(
        space0,
        do_parse!(
            preceded!(char_no_case!('O'), tag_no_case!(ident.as_str())) >>
            tag_no_case!("else") >>
            trailing_comment: opt!(comment) >>
            line_ending >>
            lines: many0!(line) >>
            ({
                Branch {
                    branch_type: BranchType::Else,
                    condition: None,
                    lines,
                    trailing_comment,
                }
            })
        )
    )
);

named!(pub conditional<Span, Conditional>,
    sep!(
        space0,
        // TODO: Extract out into some kind of named_args macro
        do_parse!(
            block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
            tag_no_case!("if") >>
            condition: gcode_expression >>
            trailing_comment: opt!(comment) >>
            line_ending >>
            lines: many0!(line) >>
            elseifs: many0!(call!(elseif, block_ident.to_ident_string())) >>
            else_block: opt!(call!(else_block, block_ident.to_ident_string())) >>
            preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
            tag_no_case!("endif") >>
            ({
                let mut branches = vec![Branch {
                    branch_type: BranchType::If,
                    condition: Some(condition),
                    lines,
                    trailing_comment
                }];

                branches.append(&mut elseifs.clone());

                if let Some(e) = else_block {
                    branches.push(e);
                }

                Conditional { identifier: block_ident, branches }
            })
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Comment, Feedrate, Token, TokenType};
    use common::{assert_parse, empty_span, span};
    use expression::{BinaryOperator, ExpressionToken, Value};

    #[test]
    fn trailing_comment() {
        assert_parse!(
            parser = conditional;
            input = span!(b"o1 if [1 gt 0] ; comment here\nf500\no1 endif");
            expected = Conditional {
                identifier: Parameter::Numbered(1),
                branches: vec![
                    Branch {
                        trailing_comment: Some(Comment { text: "comment here".into() }),
                        branch_type: BranchType::If,
                        condition: Some(Expression::from_tokens(vec![
                            ExpressionToken::Literal(1.0),
                            ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                            ExpressionToken::Literal(0.0),
                        ])),
                        lines: vec![
                            Line {
                                block_delete: false,
                                span: empty_span!(offset = 30, line = 2),
                                tokens: vec![
                                    Token {
                                        span: empty_span!(offset = 30, line = 2),
                                        token: TokenType::Feedrate(Feedrate { feedrate: Value::Float(500.0) })
                                    },
                                ]
                            }
                        ]
                    }
                ]
            };
            remaining = empty_span!(offset = 43, line = 3)
        );
    }

    #[test]
    fn parse_if() {
        assert_parse!(
            parser = conditional;
            input = span!(b"o1 if [1 gt 0]\nf500\no1 endif");
            expected = Conditional {
                identifier: Parameter::Numbered(1),
                branches: vec![
                    Branch {
                        trailing_comment: None,
                        branch_type: BranchType::If,
                        condition: Some(Expression::from_tokens(vec![
                            ExpressionToken::Literal(1.0),
                            ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                            ExpressionToken::Literal(0.0),
                        ])),
                        lines: vec![Line {
                            block_delete: false,
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

    #[test]
    fn parse_if_elseif() {
        assert_parse!(
            parser = conditional;
            input = span!(b"o1 if [1 gt 0]\nf500\no1 elseif [2 lt 3]\nf400\no1 endif");
            expected = Conditional {
                identifier: Parameter::Numbered(1),
                branches: vec![
                    Branch {
                        trailing_comment: None,
                        branch_type: BranchType::If,
                        condition: Some(Expression::from_tokens(vec![
                            ExpressionToken::Literal(1.0),
                            ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                            ExpressionToken::Literal(0.0),
                        ])),
                        lines: vec![Line {
                            block_delete: false,
                            span: empty_span!(offset = 15, line = 2),
                            tokens: vec![
                                Token {
                                    span: empty_span!(offset = 15, line = 2),
                                    token: TokenType::Feedrate(Feedrate { feedrate: Value::Float(500.0) })
                                },
                            ]
                        }]
                    },
                    Branch {
                        trailing_comment: None,
                        branch_type: BranchType::ElseIf,
                        condition: Some(Expression::from_tokens(vec![
                            ExpressionToken::Literal(2.0),
                            ExpressionToken::BinaryOperator(BinaryOperator::LessThan),
                            ExpressionToken::Literal(3.0),
                        ])),
                        lines: vec![Line {
                            block_delete: false,
                            span: empty_span!(offset = 39, line = 4),
                            tokens: vec![
                                Token {
                                    span: empty_span!(offset = 39, line = 4),
                                    token: TokenType::Feedrate(Feedrate { feedrate: Value::Float(400.0) })
                                },
                            ]
                        }]
                    }
                ]
            };
            remaining = empty_span!(offset = 52, line = 5)
        );
    }

    #[test]
    fn parse_if_elseif_else() {
        assert_parse!(
            parser = conditional;
            input = span!(b"o1 if [1 gt 0]\nf500\no1 elseif [2 lt 3]\nf400\no1 else\nf200\no1 endif");
            expected = Conditional {
                identifier: Parameter::Numbered(1),
                branches: vec![
                    Branch {
                        trailing_comment: None,
                        branch_type: BranchType::If,
                        condition: Some(Expression::from_tokens(vec![
                            ExpressionToken::Literal(1.0),
                            ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                            ExpressionToken::Literal(0.0),
                        ])),
                        lines: vec![Line {
                            block_delete: false,
                            span: empty_span!(offset = 15, line = 2),
                            tokens: vec![
                                Token {
                                    span: empty_span!(offset = 15, line = 2),
                                    token: TokenType::Feedrate(Feedrate { feedrate: Value::Float(500.0) })
                                },
                            ]
                        }]
                    },
                    Branch {
                        trailing_comment: None,
                        branch_type: BranchType::ElseIf,
                        condition: Some(Expression::from_tokens(vec![
                            ExpressionToken::Literal(2.0),
                            ExpressionToken::BinaryOperator(BinaryOperator::LessThan),
                            ExpressionToken::Literal(3.0),
                        ])),
                        lines: vec![Line {
                            block_delete: false,
                            span: empty_span!(offset = 39, line = 4),
                            tokens: vec![
                                Token {
                                    span: empty_span!(offset = 39, line = 4),
                                    token: TokenType::Feedrate(Feedrate { feedrate: Value::Float(400.0) })
                                },
                            ]
                        }]
                    },
                    Branch {
                        trailing_comment: None,
                        branch_type: BranchType::Else,
                        condition: None,
                        lines: vec![Line {
                            block_delete: false,
                            span: empty_span!(offset = 52, line = 6),
                            tokens: vec![
                                Token {
                                    span: empty_span!(offset = 52, line = 6),
                                    token: TokenType::Feedrate(Feedrate { feedrate: Value::Float(200.0) })
                                },
                            ]
                        }]
                    }
                ]
            };
            remaining = empty_span!(offset = 65, line = 7)
        );
    }
}
