use crate::line::{line, Line};
use crate::token::{comment, Comment};
use expression::{
    gcode::{expression, parameter},
    Expression, Parameter,
};
use nom::{
    bytes::streaming::{tag, tag_no_case},
    character::streaming::line_ending,
    combinator::{map, opt, recognize},
    error::{context, ParseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
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
    identifier: Parameter,
    branches: Vec<Branch>,
}

// named_args!(elseif(ident: String)<Span, Branch>,
//     sep!(
//         space0,
//         do_parse!(
//             preceded!(char_no_case!('O'), tag_no_case!(ident.as_str())) >>
//             tag_no_case!("elseif") >>
//             condition: gcode_expression >>
//             trailing_comment: opt!(comment) >>
//             line_ending >>
//             lines: many0!(line) >>
//             ({
//                 Branch {
//                     branch_type: BranchType::ElseIf,
//                     condition: Some(condition),
//                     lines,
//                     trailing_comment,
//                 }
//             })
//         )
//     )
// );

// TODO: Use conditional_block_open
pub fn elseif_block<'a, E: ParseError<&'a str>>(
    ident: &'a str,
) -> impl Fn(&'a str) -> IResult<&'a str, Branch, E> {
    context(
        "elseif branch",
        map(
            tuple((
                preceded(tag_no_case("O"), tag(ident)),
                tag_no_case("elseif"),
                expression,
                opt(comment),
                line_ending,
                many0(line),
            )),
            |(_, _, condition, trailing_comment, _, lines)| Branch {
                branch_type: BranchType::ElseIf,
                condition: Some(condition),
                lines,
                trailing_comment,
            },
        ),
    )
}

// pub fn elseif<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Branch, E> {

// }

// named_args!(else_block(ident: String)<Span, Branch>,
//     sep!(
//         space0,
//         do_parse!(
//             preceded!(char_no_case!('O'), tag_no_case!(ident.as_str())) >>
//             tag_no_case!("else") >>
//             trailing_comment: opt!(comment) >>
//             line_ending >>
//             lines: many0!(line) >>
//             ({
//                 Branch {
//                     branch_type: BranchType::Else,
//                     condition: None,
//                     lines,
//                     trailing_comment,
//                 }
//             })
//         )
//     )
// );

// TODO: Use block_open
pub fn else_block<'a, E: ParseError<&'a str>>(
    ident: &'a str,
) -> impl Fn(&'a str) -> IResult<&'a str, Branch, E> {
    context(
        "else branch",
        map(
            tuple((
                preceded(tag_no_case("O"), tag(ident)),
                tag_no_case("else"),
                opt(comment),
                line_ending,
                many0(line),
            )),
            |(_, _, trailing_comment, _, lines)| Branch {
                branch_type: BranchType::Else,
                condition: None,
                lines,
                trailing_comment,
            },
        ),
    )
}

// named!(pub conditional<Span, Conditional>,
//     sep!(
//         space0,
//         // TODO: Extract out into some kind of named_args macro
//         do_parse!(
//             block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
//             tag_no_case!("if") >>
//             condition: gcode_expression >>
//             trailing_comment: opt!(comment) >>
//             line_ending >>
//             lines: many0!(line) >>
//             elseifs: many0!(call!(elseif, block_ident.to_ident_string())) >>
//             else_block: opt!(call!(else_block, block_ident.to_ident_string())) >>
//             preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
//             tag_no_case!("endif") >>
//             ({
//                 let mut branches = vec![Branch {
//                     branch_type: BranchType::If,
//                     condition: Some(condition),
//                     lines,
//                     trailing_comment
//                 }];

//                 branches.append(&mut elseifs.clone());

//                 if let Some(e) = else_block {
//                     branches.push(e);
//                 }

//                 Conditional { identifier: block_ident, branches }
//             })
//         )
//     )
// );

// TODO: Use conditional_block_open
pub fn conditional<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Conditional, E> {
    // TODO: gcode_non_global_ident instead of `parameter`. Call it `condition_ident` or `block_ident`?
    let (i, ident) = delimited(tag_no_case("O"), recognize(parameter), tag_no_case("if"))(i)?;

    // let ident_str = ident.to_string();

    let (i, (if_block_condition, if_block_comment)) =
        terminated(tuple((expression, opt(comment))), line_ending)(i)?;

    let (i, if_block_lines) = many0(line)(i)?;

    // TODO: DRY up `.to_string().as_str()` calls that are all over the place
    let (i, elseifs) = many0(elseif_block(&ident))(i)?;

    let (i, else_block) = opt(else_block(&ident))(i)?;

    // Closing tag
    let (i, _) = context(
        "closing",
        tuple((tag_no_case("O"), tag(ident), tag_no_case("endif"))),
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

    let (_, identifier) = parameter(ident)?;

    Ok((
        i,
        Conditional {
            identifier,
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
    fn parse_if_elseif() {
        assert_parse!(
            parser = conditional;
            input = "o1 if [1 gt 0]\nf500\no1 elseif [2 lt 3]\nf400\no1 endif";
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
