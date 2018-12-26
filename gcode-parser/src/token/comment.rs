use crate::Span;
use nom::*;

/// A comment
///
/// Whitespace is trimmed from either end of the comment during parse.
#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    /// The comment text
    pub text: String,
}

named!(pub(crate) comment<Span, Comment>,
    map!(
        flat_map!(
            alt_complete!(
                delimited!(
                    char!('('),
                    take_until!(")"),
                    char!(')')
                ) |
                delimited!(
                    char!(';'),
                    take_until_either!("\r\n"),
                    line_ending
                )
            ),
            parse_to!(String)
        ),
        |text| Comment { text: text.trim().to_string() }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_parens_comment() {
        assert_parse!(
            parser = comment,
            input = span!(b"( some comment text )"),
            expected = Comment {
                text: "some comment text".into()
            },
            remaining = empty_span!(offset = 21)
        );

        // TODO: Macro to take a list of inputs and expected outputs
        assert_parse!(
            parser = comment,
            input = span!(b"(some comment text)"),
            expected = Comment {
                text: "some comment text".into()
            },
            remaining = empty_span!(offset = 19)
        );
    }

    #[test]
    fn parse_line_comment() {
        assert_parse!(
            parser = comment,
            input = span!(b"; Some comment text\n"),
            expected = Comment {
                text: "Some comment text".into()
            },
            remaining = empty_span!(offset = 20, line = 2)
        );

        // TODO: Macro to take a list of inputs and expected outputs
        assert_parse!(
            parser = comment,
            input = span!(b";Some comment text\n"),
            expected = Comment {
                text: "Some comment text".into()
            },
            remaining = empty_span!(offset = 19, line = 2)
        );
    }

    #[test]
    fn parse_widows_line_endings() {
        assert_parse!(
            parser = comment,
            input = span!(b";Some comment text\r\n"),
            expected = Comment {
                text: "Some comment text".into()
            },
            remaining = empty_span!(offset = 20, line = 2)
        );
    }
}
