mod conditional;

use self::conditional::conditional;
pub use self::conditional::{Branch, BranchType, Conditional};
use crate::line::{line, Line};
use common::parsing::Span;
use expression::parser::{gcode_expression, gcode_non_global_ident};
use expression::{Expression, Parameter};
use nom::*;

/// A control flow block
#[derive(Debug, PartialEq, Clone)]
pub enum Block<'a> {
    /// An if/elseif/else statement
    Conditional(Conditional<'a>),

    /// A do-while loop
    DoWhile(DoWhile<'a>),

    /// A while loop
    While(While<'a>),

    /// A repeat loop
    Repeat(Repeat<'a>),

    /// A subroutine
    Subroutine(Subroutine<'a>),
}

/// A do-while loop
#[derive(Debug, PartialEq, Clone)]
pub struct DoWhile<'a> {
    identifier: Parameter,
    condition: Expression,
    lines: Vec<Line<'a>>,
}

/// A while loop
#[derive(Debug, PartialEq, Clone)]
pub struct While<'a> {
    identifier: Parameter,
    condition: Expression,
    lines: Vec<Line<'a>>,
}

/// A block that is repeated _n_ times
#[derive(Debug, PartialEq, Clone)]
pub struct Repeat<'a> {
    identifier: Parameter,
    condition: Expression,
    lines: Vec<Line<'a>>,
}

/// A subroutine definition
#[derive(Debug, PartialEq, Clone)]
pub struct Subroutine<'a> {
    identifier: Parameter,
    lines: Vec<Line<'a>>,
}

named!(pub while_block<Span, While>,
    sep!(
        space0,
        // TODO: Extract out into some kind of named_args macro
        do_parse!(
            block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
            tag_no_case!("while") >>
            condition: gcode_expression >>
            lines: many0!(line) >>
            preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
            tag_no_case!("endwhile") >>
            ({
                While { identifier: block_ident, condition, lines }
            })
        )
    )
);

named!(pub do_while_block<Span, DoWhile>,
    sep!(
        space0,
        // TODO: Extract out into some kind of named_args macro
        do_parse!(
            block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
            tag_no_case!("do") >>
            lines: many0!(line) >>
            preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
            tag_no_case!("while") >>
            condition: gcode_expression >>
            ({
                DoWhile { identifier: block_ident, condition, lines }
            })
        )
    )
);

named!(pub repeat_block<Span, Repeat>,
    sep!(
        space0,
        // TODO: Extract out into some kind of named_args macro
        do_parse!(
            block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
            tag_no_case!("repeat") >>
            condition: gcode_expression >>
            lines: many0!(line) >>
            preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
            tag_no_case!("endrepeat") >>
            ({
                Repeat { identifier: block_ident, condition, lines }
            })
        )
    )
);

named!(pub subroutine<Span, Subroutine>,
    sep!(
        space0,
        // TODO: Extract out into some kind of named_args macro
        do_parse!(
            block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
            tag_no_case!("sub") >>
            lines: many0!(line) >>
            preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
            tag_no_case!("endsub") >>
            ({
                Subroutine { identifier: block_ident, lines }
            })
        )
    )
);

named!(pub block<Span, Block>,
    alt!(
        map!(conditional, |conditional| Block::Conditional(conditional)) |
        map!(while_block, |while_block| Block::While(while_block)) |
        map!(do_while_block, |do_while_block| Block::DoWhile(do_while_block)) |
        map!(repeat_block, |repeat_block| Block::Repeat(repeat_block)) |
        map!(subroutine, |subroutine| Block::Subroutine(subroutine))
    )
);
