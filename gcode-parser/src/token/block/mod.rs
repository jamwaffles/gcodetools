mod conditional;

use self::conditional::conditional;
pub use self::conditional::{Branch, BranchType, Conditional};
use crate::line::{line, Line};
use crate::token::{comment, Comment};
use expression::{gcode::expression, Expression};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{digit1, line_ending},
    combinator::{map, opt, recognize},
    error::{context, ParseError},
    multi::many0,
    sequence::{preceded, terminated, tuple},
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

/// A block identifier like `O100` or `o110`
#[derive(Debug, PartialEq, Clone)]
pub struct BlockIdent {
    name: String,
}

impl From<&str> for BlockIdent {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for BlockIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
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

pub fn block_ident<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, BlockIdent, E> {
    map(
        recognize(preceded(tag_no_case("O"), digit1)),
        |name: &'a str| BlockIdent {
            name: name.to_string(),
        },
    )(i)
}

// named!(pub while_block<Span, While>,
//     sep!(
//         space0,
//         // TODO: Extract out into some kind of named_args macro
//         do_parse!(
//             block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
//             tag_no_case!("while") >>
//             condition: gcode_expression >>
//             trailing_comment: opt!(comment) >>
//             line_ending >>
//             lines: many0!(line) >>
//             preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
//             tag_no_case!("endwhile") >>
//             ({
//                 While { identifier: block_ident, condition, lines, trailing_comment }
//             })
//         )
//     )
// );

pub fn while_block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, While, E> {
    let (i, ident) = terminated(block_ident, tag_no_case("while"))(i)?;

    let (i, (block_condition, block_comment)) =
        terminated(tuple((expression, opt(comment))), line_ending)(i)?;

    let (i, block_lines) = many0(line)(i)?;

    tuple((
        tag_no_case("O"),
        tag(ident.to_string().as_str()),
        tag_no_case("endwhile"),
    ))(i)?;

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

// named!(pub do_while_block<Span, DoWhile>,
//     sep!(
//         space0,
//         // TODO: Extract out into some kind of named_args macro
//         do_parse!(
//             block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
//             tag_no_case!("do") >>
//             lines: many0!(line) >>
//             preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
//             tag_no_case!("while") >>
//             condition: gcode_expression >>
//             ({
//                 DoWhile { identifier: block_ident, condition, lines }
//             })
//         )
//     )
// );

pub fn do_while_block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, DoWhile, E> {
    let (i, ident) = terminated(block_ident, tag_no_case("do"))(i)?;

    let (i, block_lines) = many0(line)(i)?;

    // let (i, (block_condition, block_comment)) =
    //     terminated(tuple((expression, opt(comment))), line_ending)(i)?;

    let (i, block_condition) = preceded(
        tuple((
            tag_no_case("O"),
            tag(ident.to_string().as_str()),
            tag_no_case("while"),
        )),
        expression,
    )(i)?;

    Ok((
        i,
        DoWhile {
            identifier: ident,
            condition: block_condition,
            lines: block_lines,
        },
    ))
}

// named!(pub repeat_block<Span, Repeat>,
//     sep!(
//         space0,
//         // TODO: Extract out into some kind of named_args macro
//         do_parse!(
//             block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
//             tag_no_case!("repeat") >>
//             condition: gcode_expression >>
//             trailing_comment: opt!(comment) >>
//             line_ending >>
//             lines: many0!(line) >>
//             preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
//             tag_no_case!("endrepeat") >>
//             ({
//                 Repeat { identifier: block_ident, condition, lines, trailing_comment }
//             })
//         )
//     )
// );

pub fn repeat_block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Repeat, E> {
    let (i, ident) = terminated(block_ident, tag_no_case("repeat"))(i)?;

    let (i, (block_condition, block_comment)) =
        terminated(tuple((expression, opt(comment))), line_ending)(i)?;

    let (i, block_lines) = many0(line)(i)?;

    tuple((
        tag_no_case("O"),
        tag(ident.to_string().as_str()),
        tag_no_case("endrepeat"),
    ))(i)?;

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

// named!(pub subroutine<Span, Subroutine>,
//     sep!(
//         space0,
//         // TODO: Extract out into some kind of named_args macro
//         do_parse!(
//             block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
//             tag_no_case!("sub") >>
//             trailing_comment: opt!(comment) >>
//             line_ending >>
//             lines: many0!(line) >>
//             preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
//             tag_no_case!("endsub") >>
//             returns: opt!(gcode_expression) >>
//             ({
//                 Subroutine { identifier: block_ident, lines, returns, trailing_comment }
//             })
//         )
//     )
// );

pub fn subroutine<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Subroutine, E> {
    let (i, ident) = terminated(block_ident, tag_no_case("sub"))(i)?;

    let (i, block_comment) = terminated(opt(comment), line_ending)(i)?;

    let (i, block_lines) = many0(line)(i)?;

    tuple((
        tag_no_case("O"),
        tag(ident.to_string().as_str()),
        tag_no_case("endsub"),
    ))(i)?;

    let (i, returns) = opt(expression)(i)?;

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

// named!(pub block<Span, Block>,
//     alt!(
//         map!(conditional, |conditional| Block::Conditional(conditional)) |
//         map!(while_block, |while_block| Block::While(while_block)) |
//         map!(do_while_block, |do_while_block| Block::DoWhile(do_while_block)) |
//         map!(repeat_block, |repeat_block| Block::Repeat(repeat_block)) |
//         map!(subroutine, |subroutine| Block::Subroutine(subroutine))
//     )
// );

pub fn block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Block, E> {
    context(
        "block",
        alt((
            map(conditional, Block::Conditional),
            map(while_block, Block::While),
            map(do_while_block, Block::DoWhile),
            map(repeat_block, Block::Repeat),
            map(subroutine, Block::Subroutine),
        )),
    )(i)
}
