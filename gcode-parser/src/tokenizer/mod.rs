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
pub enum CutterCompensation {
    Off,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Units(Units),
    DistanceMode(DistanceMode),
    PathBlending(PathBlending),
    CutterCompensation(CutterCompensation),
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

named!(cutter_compensation<CompleteByteSlice, Token>,
    map!(
        alt!(
            map!(tag_no_case!("G40"), |_| CutterCompensation::Off)
        ),
        |res| Token::CutterCompensation(res)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    fn check_token(
        to_check: Result<(CompleteByteSlice, Token), nom::Err<CompleteByteSlice>>,
        against: Token,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    #[test]
    fn it_parses_cutter_comp() {
        check_token(
            cutter_compensation(Cbs(b"G40")),
            Token::CutterCompensation(CutterCompensation::Off),
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
    fn it_parses_blending_mode() {
        check_token(
            path_blending(Cbs(b"G64")),
            Token::PathBlending(PathBlending { p: None, q: None }),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01")),
            Token::PathBlending(PathBlending {
                p: Some(0.01f32),
                q: None,
            }),
        );

        check_token(
            path_blending(Cbs(b"G64 P0.01 Q0.02")),
            Token::PathBlending(PathBlending {
                p: Some(0.01f32),
                q: Some(0.02f32),
            }),
        );

        // TODO
        // check_token(
        //     path_blending(Cbs(b"G64 Q0.02")),
        //     Token::PathBlending(PathBlending { p: None, q: None })
        // );
    }

    #[test]
    fn it_parses_distance_mode() {
        check_token(
            distance_mode(Cbs(b"G90")),
            Token::DistanceMode(DistanceMode::Absolute),
        );

        check_token(
            distance_mode(Cbs(b"G91")),
            Token::DistanceMode(DistanceMode::Incremental),
        );
    }
    #[test]
    fn it_parses_units() {
        check_token(units(Cbs(b"G20")), Token::Units(Units::Inch));
        check_token(units(Cbs(b"G21")), Token::Units(Units::Mm));
    }

    #[test]
    fn it_parses_comments() {
        check_token(
            comment(Cbs(b"(Hello world)")),
            Token::Comment("Hello world".into()),
        );

        check_token(
            comment(Cbs(b"( Hello world )")),
            Token::Comment("Hello world".into()),
        );
    }
}
