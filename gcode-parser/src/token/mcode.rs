use crate::value::{preceded_value, Value};
use expression::parser::gcode;
use nom::{
    branch::{alt, permutation},
    bytes::streaming::{tag, tag_no_case, take_until},
    character::streaming::{char, digit1, multispace0, space0},
    combinator::{map, map_opt, opt},
    error::{context, ParseError},
    multi::many0,
    number::streaming::float,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

/// An M-code
#[derive(Debug, PartialEq, Clone)]
pub enum MCode {
    /// Turn the spindle clockwise
    SpindleForward,

    /// Turn the spindle counter-clockwise
    SpindleReverse,

    /// Stop the spindle
    SpindleStop,

    /// Change tool
    ToolChange,

    /// End program
    EndProgram,

    /// Swap pallets and end program (M30)
    EndProgramSwapPallets,

    /// Optional pause (M1)
    OptionalPause,
}

// named!(pub mcode<Span, MCode>,
//     alt!(
//         map_code!("M1", |_| MCode::OptionalPause) |
//         map_code!("M2", |_| MCode::EndProgram) |
//         map_code!("M3", |_| MCode::SpindleForward) |
//         map_code!("M4", |_| MCode::SpindleReverse) |
//         map_code!("M5", |_| MCode::SpindleStop) |
//         map_code!("M6", |_| MCode::ToolChange) |
//         map_code!("M30", |_| MCode::EndProgramSwapPallets)
//     )
// );

pub fn mcode<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, MCode, E> {
    context(
        "M code",
        alt((
            map(tag_no_case("M1"), |_| MCode::OptionalPause),
            map(tag_no_case("M2"), |_| MCode::EndProgram),
            map(tag_no_case("M3"), |_| MCode::SpindleForward),
            map(tag_no_case("M4"), |_| MCode::SpindleReverse),
            map(tag_no_case("M5"), |_| MCode::SpindleStop),
            map(tag_no_case("M6"), |_| MCode::ToolChange),
            map(tag_no_case("M30"), |_| MCode::EndProgramSwapPallets),
        )),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;

    #[test]
    fn parse_spindle_commands() {
        assert_parse!(
            parser = mcode;
            input = "M3";
            expected = MCode::SpindleForward
        );

        assert_parse!(
            parser = mcode;
            input = "M4";
            expected = MCode::SpindleReverse
        );

        assert_parse!(
            parser = mcode;
            input = "M5";
            expected = MCode::SpindleStop
        );
    }
}
