use crate::parsers::char_no_case;
use crate::value::{preceded_decimal_value, Value};
use crate::word::word;
use nom::{
    branch::alt,
    character::complete::space0,
    combinator::{map, opt},
    error::{context, ParseError},
    sequence::{pair, preceded},
    IResult,
};

/// Cutter compensation type
#[derive(Debug, PartialEq, Clone)]
pub enum CutterCompensation {
    /// Turn cutter compensation off (G40)
    Off,

    /// Offset the tool to the left of the path (G41)
    Left(Option<Value>),

    /// Offset the tool to the right of the path (G42)
    Right(Option<Value>),
}

// named!(pub cutter_compensation<Span, CutterCompensation>,
//     alt!(
//         map_code!(
//             "G40",
//             |_| CutterCompensation::Off
//         ) |
//         map_code!(
//             "G41",
//             opt!(
//                 preceded!(
//                     char_no_case!('D'),
//                     ngc_float_value
//                 )
//             ),
//             |dia| CutterCompensation::Left(dia)
//         ) |
//         map_code!(
//             "G42",
//             opt!(
//                 preceded!(
//                     char_no_case!('D'),
//                     ngc_float_value
//                 )
//             ),
//             |dia| CutterCompensation::Right(dia)
//         )
//     )
// );

pub fn cutter_compensation<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, CutterCompensation, E> {
    context(
        "cutter comp",
        alt((
            map(word("g40"), |_| CutterCompensation::Off),
            map(
                preceded(
                    pair(word("g41"), space0),
                    opt(preceded_decimal_value(char_no_case('d'))),
                ),
                CutterCompensation::Left,
            ),
            map(
                preceded(
                    pair(word("g42"), space0),
                    opt(preceded_decimal_value(char_no_case('d'))),
                ),
                CutterCompensation::Right,
            ),
        )),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;

    #[test]
    fn parse_cutter_compensation() {
        assert_parse!(
            parser = cutter_compensation;
            input =
                "G40",
                "G41",
                "G42"
            ;
            expected =
                CutterCompensation::Off,
                CutterCompensation::Left(None),
                CutterCompensation::Right(None)
            ;
        );
    }

    #[test]
    fn parse_offsets() {
        assert_parse!(
            parser = cutter_compensation;
            input =
                "G41 D5.0",
                "G42d10.1"
            ;
            expected =
                CutterCompensation::Left(Some(5.0.into())),
                CutterCompensation::Right(Some(10.1.into()))
            ;
        );
    }
}
