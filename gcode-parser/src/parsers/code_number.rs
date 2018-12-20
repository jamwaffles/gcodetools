use crate::Span;
use nom::*;

pub type CodeNumber = f32;

named!(pub code_number<Span, CodeNumber>,
    flat_map!(
        recognize!(
            terminated!(
                digit1,
                opt!(terminated!(char!('.'), digit1))
            )
        ),
        parse_to!(f32)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_decimal() {
        assert_parse!(
            parser = code_number,
            input = span!(b"59.1"),
            expected = 59.1f32,
            remaining = empty_span!(offset = 4)
        );
    }

    #[test]
    fn parse_int() {
        assert_parse!(
            parser = code_number,
            input = span!(b"54"),
            expected = 54.0f32,
            remaining = empty_span!(offset = 2)
        );
    }
}
