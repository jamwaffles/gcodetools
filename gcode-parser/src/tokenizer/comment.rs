use nom::types::CompleteByteSlice;

use super::helpers::take_until_line_ending;
use super::Token;

named!(bracketed_comment<CompleteByteSlice, Token>, map!(
    flat_map!(
        delimited!(tag!("("), take_until!(")"), tag!(")")),
        parse_to!(String)
    ),
    |res| Token::Comment(res.trim().into())
));

named!(semicolon_comment<CompleteByteSlice, Token>, map!(
    flat_map!(
        preceded!(tag!(";"), take_until_line_ending),
        parse_to!(String)
    ),
    |res| Token::Comment(res.trim().into())
));

named!(pub comment<CompleteByteSlice, Token>,
    alt_complete!(bracketed_comment | semicolon_comment)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    fn check_token(
        to_check: Result<(CompleteByteSlice, Token), nom::Err<CompleteByteSlice>>,
        against: Token,
    ) {
        assert_eq!(to_check, Ok((EMPTY, against)))
    }

    const EMPTY: Cbs = Cbs(b"");

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

        assert_eq!(
            comment(Cbs(b"; Hello world\n")),
            Ok((Cbs(b"\n"), Token::Comment("Hello world".into())))
        );
        assert_eq!(
            comment(Cbs(b";Hello world\n")),
            Ok((Cbs(b"\n"), Token::Comment("Hello world".into())))
        );
    }
}
