use common::parsing::Span;
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
                preceded!(
                    char!(';'),
                    alt!(take_until!("\r\n") | take_until!("\n"))
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
    use common::{assert_parse, span};

    #[test]
    fn parse_parens_comment() {
        assert_parse!(
            parser = comment;
            input = span!(b"( some comment text )");
            expected = Comment {
                text: "some comment text".into()
            }
        );

        // TODO: Macro to take a list of inputs and expected outputs
        assert_parse!(
            parser = comment;
            input = span!(b"(some comment text)");
            expected = Comment {
                text: "some comment text".into()
            }
        );
    }

    #[test]
    fn parse_line_comment() {
        assert_parse!(
            parser = comment;
            input = span!(b"; Some comment text\n");
            expected = Comment {
                text: "Some comment text".into()
            };
            remaining = span!(b"\n", offset = 19, line = 1)
        );

        // TODO: Macro to take a list of inputs and expected outputs
        assert_parse!(
            parser = comment;
            input = span!(b";Some comment text\n");
            expected = Comment {
                text: "Some comment text".into()
            };
            remaining = span!(b"\n", offset = 18, line = 1)
        );
    }

    #[test]
    fn parse_windows_line_endings() {
        assert_parse!(
            parser = comment;
            input = span!(b"; Some comment text\r\n");
            expected = Comment {
                text: "Some comment text".into()
            };
            remaining = span!(b"\r\n", offset = 19, line = 1)
        );
    }
}
