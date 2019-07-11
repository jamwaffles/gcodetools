use super::{block_close, block_close_expr, block_open_expr, BlockIdent};
use crate::line::{lines_with_newline, Line};
use crate::token::Comment;
use expression::{gcode::expression, Expression};
use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::line_ending,
    combinator::{map, opt},
    error::{context, ParseError},
    multi::many0,
    sequence::separated_pair,
    IResult,
};

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
///
/// TODO: Fix `f32` into a generic
#[derive(Debug, PartialEq, Clone)]
pub struct Branch {
    branch_type: BranchType,
    lines: Vec<Line>,
    condition: Option<Expression<f32>>,
    trailing_comment: Option<Comment>,
}

/// An if/else if/else chain
#[derive(Debug, PartialEq, Clone)]
pub struct Conditional {
    identifier: BlockIdent,
    branches: Vec<Branch>,
}

// TODO: Use conditional_block_open
pub fn elseif_block<'a, IP, IOP, E: ParseError<&'a str>>(
    ident_parser: IP,
) -> impl Fn(&'a str) -> IResult<&'a str, Branch, E>
where
    IP: Fn(&'a str) -> IResult<&'a str, IOP, E>,
{
    context(
        "elseif branch",
        map(
            separated_pair(
                block_close_expr(ident_parser, tag("elseif"), expression),
                line_ending,
                lines_with_newline,
            ),
            |((condition, trailing_comment), lines)| Branch {
                branch_type: BranchType::ElseIf,
                condition: Some(condition),
                lines,
                trailing_comment,
            },
        ),
    )
}

// TODO: Use block_open
pub fn else_block<'a, IP, IOP, E: ParseError<&'a str>>(
    ident_parser: IP,
) -> impl Fn(&'a str) -> IResult<&'a str, Branch, E>
where
    IP: Fn(&'a str) -> IResult<&'a str, IOP, E>,
{
    context(
        "else branch",
        map(
            separated_pair(
                block_close(ident_parser, tag("else")),
                line_ending,
                lines_with_newline,
            ),
            |(trailing_comment, lines)| Branch {
                branch_type: BranchType::Else,
                condition: None,
                lines,
                trailing_comment,
            },
        ),
    )
}

// TODO: Use conditional_block_open
pub fn conditional<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Conditional, E> {
    let (i, (ident, if_block_condition, if_block_comment)) = block_open_expr(tag("if"))(i)?;

    let (i, if_block_lines) = lines_with_newline(i)?;

    let (i, elseifs) = many0(elseif_block(tag_no_case(ident.to_string().as_str())))(i)?;

    let (i, else_block) = opt(else_block(tag_no_case(ident.to_string().as_str())))(i)?;

    // Closing tag
    let (i, _) = context(
        "if block close",
        block_close(tag_no_case(ident.to_string().as_str()), tag("endif")),
    )(i)?;

    let mut branches = vec![Branch {
        branch_type: BranchType::If,
        condition: Some(if_block_condition),
        lines: if_block_lines,
        trailing_comment: if_block_comment,
    }];

    branches.append(&mut elseifs.clone());

    if let Some(e) = else_block {
        branches.push(e);
    }

    Ok((
        i,
        Conditional {
            identifier: ident,
            branches,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use crate::token::{Comment, Feedrate, Token, TokenType};
    use expression::{BinaryOperator, ExpressionToken};

    #[test]
    fn trailing_comment() {
        assert_parse!(
            parser = conditional;
            input = "o1 if [1 gt 0] ; comment here\nf500\no1 endif";
            expected = Conditional {
                identifier: 1.into(),
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
                                tokens: vec![
                                    Token {
                                        token: TokenType::Feedrate(Feedrate { feedrate: 500.0.into() })
                                    },
                                ],
                                ..Line::default()
                            }
                        ]
                    }
                ]
            };
        );
    }

    #[test]
    fn parse_if() {
        assert_parse!(
            parser = conditional;
            input = "o1 if [1 gt 0]\nf500\no1 endif";
            expected = Conditional {
                identifier: 1.into(),
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
                            tokens: vec![
                                Token {
                                    token: TokenType::Feedrate(Feedrate { feedrate: 500.0.into() })
                                },
                            ],
                            ..Line::default()
                        }]
                    }
                ]
            };
        );
    }

    #[test]
    fn parse_indented_if() {
        assert_parse!(
            parser = conditional;
            input = "    o1 if [1 gt 0]\n        f500\n    o1 endif";
            expected = Conditional {
                identifier: 1.into(),
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
                            tokens: vec![
                                Token {
                                    token: TokenType::Feedrate(Feedrate { feedrate: 500.0.into() })
                                },
                            ],
                            ..Line::default()
                        }]
                    }
                ]
            };
        );
    }

    #[test]
    fn parse_if_else() {
        assert_parse!(
            parser = conditional;
            input = "o1 if [1 gt 0]\nf500\no1 else\nf400\no1 endif";
            expected = Conditional {
                identifier: 1.into(),
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
                            tokens: vec![
                                Token {
                                    token: TokenType::Feedrate(Feedrate { feedrate: 500.0.into() })
                                },
                            ],
                            ..Line::default()
                        }]
                    },
                    Branch {
                        trailing_comment: None,
                        branch_type: BranchType::Else,
                        condition: None,
                        lines: vec![Line {
                            tokens: vec![
                                Token {
                                    token: TokenType::Feedrate(Feedrate { feedrate: 400.0.into() })
                                },
                            ],
                            ..Line::default()
                        }]
                    }
                ]
            };
        );
    }

    #[test]
    fn parse_if_elseif() {
        assert_parse!(
            parser = conditional;
            input = "o1 if [1 gt 0]\nf500\no1 elseif [2 lt 3]\nf400\no1 endif";
            expected = Conditional {
                identifier: 1.into(),
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
                            tokens: vec![
                                Token {
                                    token: TokenType::Feedrate(Feedrate { feedrate: 500.0.into() })
                                },
                            ],
                            ..Line::default()
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
                            tokens: vec![
                                Token {
                                    token: TokenType::Feedrate(Feedrate { feedrate: 400.0.into() })
                                },
                            ],
                            ..Line::default()
                        }]
                    }
                ]
            };
        );
    }

    #[test]
    fn parse_if_elseif_else() {
        assert_parse!(
            parser = conditional;
            input = "o1 if [1 gt 0]\nf500\no1 elseif [2 lt 3]\nf400\no1 else\nf200\no1 endif";
            expected = Conditional {
                identifier: 1.into(),
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
                            tokens: vec![
                                Token {
                                    token: TokenType::Feedrate(Feedrate { feedrate: 500.0.into() })
                                },
                            ],
                            ..Line::default()
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
                            tokens: vec![
                                Token {
                                    token: TokenType::Feedrate(Feedrate { feedrate: 400.0.into() })
                                },
                            ],
                            ..Line::default()
                        }]
                    },
                    Branch {
                        trailing_comment: None,
                        branch_type: BranchType::Else,
                        condition: None,
                        lines: vec![Line {
                            tokens: vec![
                                Token {
                                    token: TokenType::Feedrate(Feedrate { feedrate: 200.0.into() })
                                },
                            ],
                            ..Line::default()
                        }]
                    }
                ]
            };
        );
    }
}
