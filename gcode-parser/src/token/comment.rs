use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::{char, space0},
    combinator::map,
    error::{context, ParseError},
    sequence::{delimited, preceded},
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
                preceded(
                    space0,
                    alt((
                        delimited(char('('), take_until(")"), char(')')),
                        preceded(char(';'), take_until("\n")),
                    )),
                ),
                String::from,
            ),
            |text| Comment {
                text: text.trim().to_string(),
            },
        ),
    )(i)
}

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
