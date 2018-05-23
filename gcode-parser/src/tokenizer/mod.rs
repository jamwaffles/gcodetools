use nom::types::CompleteByteSlice;

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new_from_str() -> Self {
        Tokenizer {}
    }

    pub fn tokenize(&self) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Units {
    Inch,
    Mm,
}

#[derive(Debug, PartialEq)]
pub enum DistanceMode {
    Absolute,
    Incremental,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Units(Units),
    DistanceMode(DistanceMode),
}

named!(comment<CompleteByteSlice, Token>, map!(
    flat_map!(
        delimited!(tag!("("), take_until!(")"), tag!(")")),
        parse_to!(String)
    ),
    |res| Token::Comment(res.trim().into())
));

named!(units<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("G20"), |_| Units::Inch) |
        map!(tag_no_case!("G21"), |_| Units::Mm)
    ),
    |res| Token::Units(res)
));

named!(distance_mode<CompleteByteSlice, Token>, map!(
    alt!(
        map!(tag_no_case!("G90"), |_| DistanceMode::Absolute) |
        map!(tag_no_case!("G91"), |_| DistanceMode::Incremental)
    ),
    |res| Token::DistanceMode(res)
));

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: CompleteByteSlice = CompleteByteSlice(b"");

    #[test]
    fn it_parses_distance_mode() {
        assert_eq!(
            distance_mode(CompleteByteSlice(b"G90")),
            Ok((EMPTY, Token::DistanceMode(DistanceMode::Absolute)))
        );

        assert_eq!(
            distance_mode(CompleteByteSlice(b"G91")),
            Ok((EMPTY, Token::DistanceMode(DistanceMode::Incremental)))
        );
    }

    #[test]
    fn it_parses_units() {
        assert_eq!(
            units(CompleteByteSlice(b"G20")),
            Ok((EMPTY, Token::Units(Units::Inch)))
        );

        assert_eq!(
            units(CompleteByteSlice(b"G21")),
            Ok((EMPTY, Token::Units(Units::Mm)))
        );
    }

    #[test]
    fn it_parses_comments() {
        assert_eq!(
            comment(CompleteByteSlice(b"(Hello world)")),
            Ok((EMPTY, Token::Comment("Hello world".into())))
        );

        // Make sure whitespace is trimmed
        assert_eq!(
            comment(CompleteByteSlice(b"( Hello world )")),
            Ok((EMPTY, Token::Comment("Hello world".into())))
        );
    }
}
