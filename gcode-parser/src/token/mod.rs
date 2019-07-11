//! Tokens

pub(crate) mod arc;
pub(crate) mod assignment;
pub(crate) mod block;
pub(crate) mod call;
pub(crate) mod comment;
pub(crate) mod coord;
pub(crate) mod gcode;
pub(crate) mod mcode;
pub(crate) mod othercode;
pub(crate) mod polar;
pub(crate) mod return_stmt;

use self::arc::{center_format_arc, radius_format_arc};
pub use self::arc::{CenterFormatArc, RadiusFormatArc};
use self::assignment::assignment;
pub use self::assignment::Assignment;
use self::block::block;
pub use self::block::{Block, Branch, BranchType, Conditional, Repeat, Subroutine, While};
use self::call::call;
pub use self::call::Call;
use self::comment::comment;
pub use self::comment::Comment;
use self::coord::coord;
pub use self::coord::Coord;
use self::gcode::gcode;
pub use self::gcode::{CutterCompensation, GCode, WorkOffset};
use self::mcode::mcode;
pub use self::mcode::MCode;
use self::othercode::{feedrate, spindle_speed, tool_number};
pub use self::othercode::{Feedrate, SpindleSpeed, ToolNumber};
use self::polar::polar;
pub use self::polar::PolarCoord;
use self::return_stmt::return_stmt;
pub use self::return_stmt::Return;
use crate::token::othercode::raw_line_number;
pub use crate::token::othercode::LineNumber;
use crate::value::{decimal_value, Value};
use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::map,
    error::{context, ParseError},
    sequence::tuple,
    IResult,
};

/// Any possible token type recgonised by this parser
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    /// Any G-code
    GCode(GCode),

    /// Any M-code
    MCode(MCode),

    /// A coordinate consisting of at least one XYZUVWABC component
    Coord(Coord),

    /// A polar coordinate defined by distance and angle from current position
    PolarCoord(PolarCoord),

    /// The coordinates and offsets that define a clockwise (G2) or counterclockwise (G3) center
    /// format arc
    CenterFormatArc(CenterFormatArc),

    /// Radius-format arc
    RadiusFormatArc(RadiusFormatArc),

    /// Feedrate
    Feedrate(Feedrate),

    /// Spindle speed
    SpindleSpeed(SpindleSpeed),

    /// Tool number
    ToolNumber(ToolNumber),

    /// Line number
    LineNumber(LineNumber),

    /// A comment
    Comment(Comment),

    /// A code that this parser doesn't understand
    Unknown(Unknown),

    /// An assignment of a literal, parameter or expression to a parameter
    Assignment(Assignment),

    /// A block (subrouting, while loop, if statement, etc)
    Block(Block),

    /// A subroutine call
    Call(Call<f32>),

    /// A return statement
    Return(Return),

    /// Block delete (`/` character at beginning of line)
    BlockDelete,

    /// Program delimiter (`%` character literal)
    ProgramDelimiter,
}

/// An unknown token
#[derive(Debug, PartialEq, Clone)]
pub struct Unknown {
    /// Code letter (`'G'`, `'M'`, etc)
    pub code_letter: char,

    /// Code number
    pub code_number: Value,
}

/// Parsed GCode token
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// Position in the source file at which this token occurs
    // TODO: Re-enable
    // pub span: Span<'a>,

    /// The type and value of this token
    pub token: TokenType,
}

/// Parse an unknown token into its letter and numeric code parts
pub fn unknown<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Unknown, E> {
    context(
        "unknown token",
        map(
            tuple((
                one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                decimal_value,
            )),
            |(code_letter, code_number)| Unknown {
                code_letter,
                code_number,
            },
        ),
    )(i)
}

/// Parse a token into a `TokenType` enum
pub fn token_type<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, TokenType, E> {
    alt((
        map(center_format_arc, TokenType::CenterFormatArc),
        map(radius_format_arc, TokenType::RadiusFormatArc),
        map(coord, TokenType::Coord),
        map(gcode, TokenType::GCode),
        map(mcode, TokenType::MCode),
        map(feedrate, TokenType::Feedrate),
        map(spindle_speed, TokenType::SpindleSpeed),
        map(tool_number, TokenType::ToolNumber),
        map(comment, TokenType::Comment),
        map(assignment, TokenType::Assignment),
        map(block, TokenType::Block),
        map(call, TokenType::Call),
        map(return_stmt, TokenType::Return),
        map(polar, TokenType::PolarCoord),
        map(char('%'), |_| TokenType::ProgramDelimiter),
        map(unknown, TokenType::Unknown),
    ))(i)
}

/// Parse a token
pub fn token<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    map(token_type, |token| Token { token })(i)
}

/// Parse block delete
pub fn block_delete<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    map(char('/'), |_| Token {
        token: TokenType::BlockDelete,
    })(i)
}

/// Parse a line number
pub fn line_number<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    map(raw_line_number, |n| Token {
        token: TokenType::LineNumber(n),
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_code() {
        assert_parse!(
            parser = token_type;
            input = "M61", "g33";
            expected = TokenType::Unknown(Unknown {
                code_letter: 'M',
                code_number: Value::Literal(61.0)
            }),
            TokenType::Unknown(Unknown {
                code_letter: 'g',
                code_number: Value::Literal(33.0)
            })
        );
    }
}
