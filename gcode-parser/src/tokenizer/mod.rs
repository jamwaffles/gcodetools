mod helpers;

use nom::types::CompleteByteSlice;

use self::helpers::*;

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
pub struct PathBlending {
    pub p: Option<f32>,
    pub q: Option<f32>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Units(Units),
    DistanceMode(DistanceMode),
    PathBlending(PathBlending),
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

named!(path_blending<CompleteByteSlice, Token>, ws!(
    do_parse!(
        tag_no_case!("G64") >>
        p: opt!(call!(preceded_f32, "P")) >>
        q: opt!(call!(preceded_f32, "Q")) >> ({
            Token::PathBlending(PathBlending { p, q })
        })
    )
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    #[test]
    fn it_parses_blending_mode() {
        assert_eq!(
            path_blending(Cbs(b"G64")),
            Ok((
                EMPTY,
                Token::PathBlending(PathBlending { p: None, q: None })
            ))
        );

        assert_eq!(
            path_blending(Cbs(b"G64 P0.01")),
            Ok((
                EMPTY,
                Token::PathBlending(PathBlending {
                    p: Some(0.01f32),
                    q: None
                })
            ))
        );

        assert_eq!(
            path_blending(Cbs(b"G64 P0.01 Q0.02")),
            Ok((
                EMPTY,
                Token::PathBlending(PathBlending {
                    p: Some(0.01f32),
                    q: Some(0.02f32)
                })
            ))
        );

        // TODO
        // assert_eq!(
        //     path_blending(Cbs(b"G64 Q0.02")),
        //     Ok((
        //         EMPTY,
        //         Token::PathBlending(PathBlending { p: None, q: None })
        //     ))
        // );
    }

    #[test]
    fn it_parses_distance_mode() {
        assert_eq!(
            distance_mode(Cbs(b"G90")),
            Ok((EMPTY, Token::DistanceMode(DistanceMode::Absolute)))
        );

        assert_eq!(
            distance_mode(Cbs(b"G91")),
            Ok((EMPTY, Token::DistanceMode(DistanceMode::Incremental)))
        );
    }

    #[test]
    fn it_parses_units() {
        assert_eq!(units(Cbs(b"G20")), Ok((EMPTY, Token::Units(Units::Inch))));
        assert_eq!(units(Cbs(b"G21")), Ok((EMPTY, Token::Units(Units::Mm))));
    }

    #[test]
    fn it_parses_comments() {
        assert_eq!(
            comment(Cbs(b"(Hello world)")),
            Ok((EMPTY, Token::Comment("Hello world".into())))
        );

        // Make sure whitespace is trimmed
        assert_eq!(
            comment(Cbs(b"( Hello world )")),
            Ok((EMPTY, Token::Comment("Hello world".into())))
        );
    }
}
