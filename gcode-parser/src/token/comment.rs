use nom::{
    branch::{alt, permutation},
    bytes::streaming::{tag, tag_no_case, take_until},
    character::streaming::{char, digit1, multispace0, space0},
    combinator::{map, map_res, opt},
    error::{context, ParseError},
    multi::many0,
    number::streaming::float,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

/// A comment
///
/// Whitespace is trimmed from either end of the comment during parse.
#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    /// The comment text
    pub text: String,
}

pub fn comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Comment, E> {
    context(
        "comment",
        map(
            map(
                alt((
                    delimited(char('('), take_until(")"), char(')')),
                    preceded(char(';'), take_until("\n")),
                )),
                String::from,
            ),
            |text| Comment {
                text: text.trim().to_string(),
            },
        ),
    )(i)
}

// named!(pub(crate) comment<Span, Comment>,
//     map!(
//         flat_map!(
//             alt!(
//                 delimited!(
//                     char!('('),
//                     take_until!(")"),
//                     char!(')')
//                 ) |
//                 preceded!(
//                     char!(';'),
//                     alt!(take_until!("\r\n") | take_until!("\n"))
//                 )
//             ),
//             parse_to!(String)
//         ),
//         |text| Comment { text: text.trim().to_string() }
//     )
// );

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;

    #[test]
    fn parse_parens_comment() {
        assert_parse!(
            parser = comment;
            input = "( some comment text )";
            expected = Comment {
                text: "some comment text".into()
            }
        );

        assert_parse!(
            parser = comment;
            input = "(some comment text)";
            expected = Comment {
                text: "some comment text".into()
            }
        );
    }

    #[test]
    fn parse_line_comment() {
        assert_parse!(
            parser = comment;
            input = "; Some comment text\n";
            expected = Comment {
                text: "Some comment text".into()
            };
            remaining = "\n"
        );

        // TODO: Macro to take a list of inputs and expected outputs
        assert_parse!(
            parser = comment;
            input = ";Some comment text\n";
            expected = Comment {
                text: "Some comment text".into()
            };
            remaining = "\n"
        );
    }

    #[test]
    fn parse_windows_line_endings() {
        assert_parse!(
            parser = comment;
            input = "; Some comment text\r\n";
            expected = Comment {
                text: "Some comment text".into()
            };
            remaining = "\n"
        );
    }
}
