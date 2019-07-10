mod conditional;

use self::conditional::conditional;
pub use self::conditional::{Branch, BranchType, Conditional};
use crate::line::{lines_with_newline, Line};
use crate::parsers::char_no_case;
use crate::token::{comment, Comment};
use expression::{gcode::expression, Expression};
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_until},
    character::complete::{char, digit1, line_ending, multispace0, space0, space1},
    combinator::{map, map_res, opt},
    error::{context, ParseError},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::fmt;

/// A control flow block
#[derive(Debug, PartialEq, Clone)]
pub enum Block {
    /// An if/elseif/else statement
    Conditional(Conditional),

    /// A do-while loop
    DoWhile(DoWhile),

    /// A while loop
    While(While),

    /// A repeat loop
    Repeat(Repeat),

    /// A subroutine
    Subroutine(Subroutine),
}

/// The type of identifier
#[derive(Debug, PartialEq, Clone)]
pub enum IdentType {
    /// A number like `o100`
    Numbered(u16),

    /// Named like `o<touchoff>`
    Named(String),
}

impl From<&str> for IdentType {
    fn from(ident: &str) -> Self {
        IdentType::Named(ident.to_string())
    }
}

impl From<u16> for IdentType {
    fn from(num: u16) -> Self {
        IdentType::Numbered(num)
    }
}

impl fmt::Display for IdentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IdentType::Numbered(n) => write!(f, "O{}", n),
            IdentType::Named(n) => write!(f, "O<{}>", n),
        }
    }
}

/// A block identifier like `O100`, `o110` or `o<touchoff>`
#[derive(Debug, PartialEq, Clone)]
pub struct BlockIdent {
    ident: IdentType,
}

impl From<&str> for BlockIdent {
    fn from(ident: &str) -> Self {
        Self {
            ident: ident.into(),
        }
    }
}

impl From<u16> for BlockIdent {
    fn from(ident: u16) -> Self {
        Self {
            ident: ident.into(),
        }
    }
}

impl fmt::Display for BlockIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

/// A do-while loop
#[derive(Debug, PartialEq, Clone)]
pub struct DoWhile {
    identifier: BlockIdent,
    condition: Expression<f32>,
    lines: Vec<Line>,
}

/// A while loop
#[derive(Debug, PartialEq, Clone)]
pub struct While {
    identifier: BlockIdent,
    condition: Expression<f32>,
    lines: Vec<Line>,
    trailing_comment: Option<Comment>,
}

/// A block that is repeated _n_ times
#[derive(Debug, PartialEq, Clone)]
pub struct Repeat {
    identifier: BlockIdent,
    condition: Expression<f32>,
    lines: Vec<Line>,
    trailing_comment: Option<Comment>,
}

/// A subroutine definition
#[derive(Debug, PartialEq, Clone)]
pub struct Subroutine {
    identifier: BlockIdent,
    lines: Vec<Line>,
    trailing_comment: Option<Comment>,
    returns: Option<Expression<f32>>,
}

// TODO: Use general purpose `numbered_ident` and `local_ident` or whatever methods
pub fn block_ident<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, BlockIdent, E> {
    preceded(
        char_no_case('O'),
        alt((
            map_res(digit1, |ident: &'a str| {
                ident
                    .parse::<u16>()
                    .map_err(|_| "Failed to parse numeric identifier")
                    .map(|ident| BlockIdent {
                        ident: IdentType::Numbered(ident),
                    })
            }),
            map(
                delimited(char('<'), take_until(">"), char('>')),
                |ident: &'a str| BlockIdent {
                    ident: IdentType::Named(ident.to_string()),
                },
            ),
        )),
    )(i)
}

pub fn while_block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, While, E> {
    let (i, ident) = delimited(
        space0,
        block_ident,
        delimited(space0, tag_no_case("while"), space1),
    )(i)?;

    let (i, (block_condition, block_comment)) = terminated(
        pair(expression, preceded(space0, opt(comment))),
        line_ending,
    )(i)?;

    let (i, block_lines) = lines_with_newline(i)?;

    let (i, _) = separated_pair(
        preceded(multispace0, tag_no_case(ident.to_string().as_str())),
        space0,
        tag_no_case("endwhile"),
    )(i)?;

    Ok((
        i,
        While {
            identifier: ident,
            condition: block_condition,
            lines: block_lines,
            trailing_comment: block_comment,
        },
    ))
}

pub fn do_while_block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, DoWhile, E> {
    let (i, ident) = terminated(delimited(space0, block_ident, space0), tag_no_case("do"))(i)?;

    let (i, _block_comment) = terminated(opt(comment), line_ending)(i)?;

    let (i, block_lines) = lines_with_newline(i)?;

    let (i, (_, _, _, _, block_condition)) = tuple((
        tag_no_case(ident.to_string().as_str()),
        space0,
        tag_no_case("while"),
        space0,
        expression,
    ))(i)?;

    Ok((
        i,
        DoWhile {
            identifier: ident,
            condition: block_condition,
            lines: block_lines,
        },
    ))
}

pub fn repeat_block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Repeat, E> {
    let (i, ident) = delimited(
        space0,
        block_ident,
        delimited(space0, tag_no_case("repeat"), space1),
    )(i)?;

    let (i, (block_condition, block_comment)) = terminated(
        pair(expression, preceded(space0, opt(comment))),
        line_ending,
    )(i)?;

    let (i, block_lines) = lines_with_newline(i)?;

    let (i, _) = preceded(
        space0,
        separated_pair(
            tag_no_case(ident.to_string().as_str()),
            space0,
            tag_no_case("endrepeat"),
        ),
    )(i)?;

    Ok((
        i,
        Repeat {
            identifier: ident,
            condition: block_condition,
            lines: block_lines,
            trailing_comment: block_comment,
        },
    ))
}

pub fn subroutine<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Subroutine, E> {
    let (i, (ident, _)) =
        separated_pair(preceded(space0, block_ident), space0, tag_no_case("sub"))(i)?;

    let (i, block_comment) = delimited(space0, opt(comment), line_ending)(i)?;

    let (i, block_lines) = lines_with_newline(i)?;

    let (i, _) = separated_pair(
        tag_no_case(ident.to_string().as_str()),
        space0,
        tag_no_case("endsub"),
    )(i)?;

    // Optional return value
    let (i, returns) = opt(preceded(space0, expression))(i)?;

    Ok((
        i,
        Subroutine {
            identifier: ident,
            lines: block_lines,
            returns: returns,
            trailing_comment: block_comment,
        },
    ))
}

pub fn block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Block, E> {
    context(
        "block",
        alt((
            map(do_while_block, Block::DoWhile),
            map(conditional, Block::Conditional),
            map(while_block, Block::While),
            map(repeat_block, Block::Repeat),
            map(subroutine, Block::Subroutine),
        )),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;
    use expression::{BinaryOperator, ExpressionToken, Parameter};

    #[test]
    fn repeat() {
        assert_parse!(
            parser = repeat_block;
            input = "o100 repeat [800]\ng91 g1 @-.0025 ^4.5\no100 endrepeat";
            expected = Repeat {
                identifier: BlockIdent { ident: 100.into() },
                condition: Expression::from_tokens(vec![
                    ExpressionToken::Literal(800.0),
                ]),
                lines: vec![Line {
                    tokens: vec![
                        Token {
                            token: TokenType::Unknown(Unknown {
                                code_letter: 'g',
                                code_number: 91.0.into(),
                            }),
                        },
                        Token {
                            token: TokenType::GCode(GCode::Feed),
                        },
                        Token {
                            token: TokenType::PolarCoord(PolarCoord {
                                distance: Some(Value::Literal(-0.0025)),
                                angle: Some(4.5.into()),
                            }),
                        },
                    ],
                }],
                trailing_comment: None,
            };
        );
    }

    #[test]
    fn test_while() {
        assert_parse!(
            parser = while_block;
            input = "o101 while [#8 GT #4]\ng0\no101 endwhile";
            expected = While {
                identifier: BlockIdent { ident: 101.into() },
                condition: Expression::from_tokens(vec![
                    ExpressionToken::Parameter(Parameter::Numbered(8)),
                    ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                    ExpressionToken::Parameter(Parameter::Numbered(4)),
                ]),
                lines: vec![Line {
                    tokens: vec![
                        Token {
                            token: TokenType::GCode(GCode::Rapid),
                        },
                    ],
                }],
                trailing_comment: None,
            };
        );
    }

    #[test]
    fn while_indented() {
        assert_parse!(
            parser = while_block;
            input = "    o101 while [#8 GT #4]\n        g0\n    o101 endwhile";
            expected = While {
                identifier: BlockIdent { ident: 101.into() },
                condition: Expression::from_tokens(vec![
                    ExpressionToken::Parameter(Parameter::Numbered(8)),
                    ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                    ExpressionToken::Parameter(Parameter::Numbered(4)),
                ]),
                lines: vec![Line { tokens: vec![Token { token: TokenType::GCode(GCode::Rapid) }] }],
                trailing_comment: None,
            };
        );
    }

    #[test]
    fn no_spaces() {
        assert_parse!(
            parser = do_while_block;
            input = "o<ident>do\ng0\no<ident>while [#8 GT #4]";
            expected = DoWhile {
                identifier: BlockIdent { ident: "ident".into() },
                condition: Expression::from_tokens(vec![
                    ExpressionToken::Parameter(Parameter::Numbered(8)),
                    ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                    ExpressionToken::Parameter(Parameter::Numbered(4)),
                ]),
                lines: vec![Line { tokens: vec![Token { token: TokenType::GCode(GCode::Rapid) }] }],
            };
        );
    }

    #[test]
    fn offsets_ngc() {
        assert_parse!(
            parser = do_while_block;
            input = "o10 do\ng0\no10 while [5 gt 2]";
            expected = DoWhile {
                identifier: BlockIdent { ident: 10.into() },
                condition: Expression::from_tokens(vec![
                    ExpressionToken::Literal(5.0),
                    ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                    ExpressionToken::Literal(2.0),
                ]),
                lines: vec![Line { tokens: vec![Token { token: TokenType::GCode(GCode::Rapid) }] }],
            };
        );
    }
}
