use crate::line::{line, Line};
use common::parsing::Span;
use expression::parser::{gcode_expression, ngc_unsigned_value};
use expression::{Expression, Value};
use nom::*;

/// Which type of block this is
#[derive(Debug, PartialEq, Clone)]
pub enum BlockType {
    /// An if statement
    If,

    /// A while loop
    While,

    /// A repeat loop
    Repeat,

    /// A subroutine
    Subroutine,
}

impl BlockType {
    /// Get the closing tag token for this block type
    pub fn closing_tag_ident(&self) -> &'static str {
        match self {
            BlockType::If => "endif",
            BlockType::While => "endwhile",
            BlockType::Repeat => "endrepeat",
            BlockType::Subroutine => "endsub",
        }
    }
}

/// A block
#[derive(Debug, PartialEq, Clone)]
pub struct Block<'a> {
    block_ident: Value,
    block_type: BlockType,
    lines: Vec<Line<'a>>,
    condition: Option<Expression>,
}

named!(block_type<Span, BlockType>,
    alt_complete!(
        map!(tag_no_case!("if"), |_| BlockType::If) |
        map!(tag_no_case!("while"), |_| BlockType::While) |
        map!(tag_no_case!("repeat"), |_| BlockType::Repeat) |
        map!(tag_no_case!("sub"), |_| BlockType::Subroutine)
    )
);

named!(pub block<Span, Block>,
    sep!(
        space0,
        do_parse!(
            block_ident: preceded!(char_no_case!('O'), ngc_unsigned_value) >>
            block_type: block_type >>
            condition: opt!(gcode_expression) >>
            lines: many0!(line) >>
            preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_string().as_str())) >>
            tag_no_case!(block_type.closing_tag_ident()) >>
            ({
                Block {
                    condition,
                    block_ident,
                    block_type,
                    lines,
                }
            })
        )
    )
);

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: Tests
}
