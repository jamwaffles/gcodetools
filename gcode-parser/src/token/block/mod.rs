mod condition;

use crate::line::{line, Line};
use common::parsing::Span;
use expression::parser::{gcode_expression, gcode_non_global_ident};
use expression::{Expression, Parameter};
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
    block_ident: Parameter,
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
            block_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
            block_type: block_type >>
            condition: opt!(gcode_expression) >>
            line_ending >>
            lines: many0!(line) >>
            preceded!(char_no_case!('O'), tag_no_case!(block_ident.to_ident_string().as_str())) >>
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
    use super::*;
    use crate::token::{GCode, Token, TokenType, WorkOffset, WorkOffsetValue};
    use common::{assert_parse, empty_span, span};
    use expression::Parameter;

    #[test]
    fn parse_sub() {
        assert_parse!(
            parser = block;
            input = span!(r#"o100 sub
                    g54
                o100 endsub"#
                .as_bytes());
            expected = Block {
                condition: None,
                block_type: BlockType::Subroutine,
                block_ident: Parameter::Numbered(100),
                lines: vec![
                    Line {
                        span: empty_span!(offset = 29, line = 2),
                        tokens: vec![
                            Token {
                                span: empty_span!(offset = 29, line = 2),
                                token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                                    offset: WorkOffsetValue::G54,
                                }))
                            }
                        ]
                    }
                ]
            };
            remaining = empty_span!(offset = 60, line = 3);
        );
    }

    #[test]
    fn parse_named_sub() {
        assert_parse!(
            parser = block;
            input = span!(r#"o<foo> sub
    g54
o<foo> endsub"#
                .as_bytes());
            expected = Block {
                condition: None,
                block_type: BlockType::Subroutine,
                block_ident: Parameter::Named("foo".into()),
                lines: vec![
                    Line {
                        span: empty_span!(offset = 15, line = 2),
                        tokens: vec![
                            Token {
                                span: empty_span!(offset = 15, line = 2),
                                token: TokenType::GCode(GCode::WorkOffset(WorkOffset {
                                    offset: WorkOffsetValue::G54,
                                }))
                            }
                        ]
                    }
                ]
            };
            remaining = empty_span!(offset = 32, line = 3);
        );
    }
}
