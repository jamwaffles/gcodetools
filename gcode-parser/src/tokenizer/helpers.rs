use nom::types::CompleteByteSlice;
use nom::*;
use nom::{need_more, Err, ErrorKind, IResult, Needed};

use super::Token;

named!(pub take_until_line_ending<CompleteByteSlice, CompleteByteSlice>, alt_complete!(take_until!("\r\n") | take_until!("\n")));

named_args!(
    pub preceded_f32<'a>(preceding: &str)<CompleteByteSlice<'a>, f32>,
    flat_map!(ws!(preceded!(tag_no_case!(preceding), recognize_float)), parse_to!(f32))
);

named_args!(
    pub preceded_u32<'a>(preceding: &str)<CompleteByteSlice<'a>, u32>,
    flat_map!(preceded!(tag_no_case!(preceding), recognize!(digit)), parse_to!(u32))
);

pub fn one_of_no_case<'a>(
    i: CompleteByteSlice<'a>,
    inp: &str,
) -> IResult<CompleteByteSlice<'a>, char> {
    let inp_lower = inp.to_ascii_lowercase();

    match i
        .iter_elements()
        .next()
        .map(|c| (c, inp_lower.as_str().find_token(c.to_ascii_lowercase())))
    {
        None => need_more(i, Needed::Size(1)),
        Some((_, false)) => Err(Err::Error(error_position!(i, ErrorKind::OneOf::<u32>))),
        //the unwrap should be safe here
        Some((c, true)) => Ok((
            i.slice(c.len()..),
            i.iter_elements().next().unwrap().as_char(),
        )),
    }
}

named_args!(char_no_case(search: char)<CompleteByteSlice, char>,
    alt!(char!(search.to_ascii_lowercase()) | char!(search.to_ascii_uppercase()))
);

named_args!(
    preceded_code<'a>(preceding: char, code: f32)<CompleteByteSlice<'a>, (char, f32)>,
    map_res!(
        flat_map!(
            preceded!(call!(char_no_case, preceding), recognize_float),
            parse_to!(f32)
        ),
        |res| {
            if res == code {
                Ok((preceding.to_ascii_uppercase(), res))
            } else {
                Err(())
            }
        }
    )
);

named_args!(
    pub g<'a>(c: f32)<CompleteByteSlice<'a>, CompleteByteSlice<'a>>,
    recognize!(call!(preceded_code, 'G', c))
);

named_args!(
    pub m<'a>(c: f32)<CompleteByteSlice<'a>, CompleteByteSlice<'a>>,
    recognize!(call!(preceded_code, 'M', c))
);

named!(pub end_program<CompleteByteSlice, Token>, map!(
    alt!(
        recognize!(call!(m, 30.0)) |
        recognize!(call!(m, 2.0)) |
        tag!("%")
    ),
    |_| Token::EndProgram
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    #[test]
    fn it_takes_until_any_line_ending() {
        assert_eq!(
            take_until_line_ending(CompleteByteSlice(b"Unix line endings\n")),
            Ok((
                CompleteByteSlice(b"\n"),
                CompleteByteSlice(b"Unix line endings")
            ))
        );

        assert_eq!(
            take_until_line_ending(CompleteByteSlice(b"Windows line endings\r\n")),
            Ok((
                CompleteByteSlice(b"\r\n"),
                CompleteByteSlice(b"Windows line endings")
            ))
        );
    }

    #[test]
    fn it_parses_preceded_floats() {
        assert_eq!(preceded_f32(Cbs(b"J0"), "J"), Ok((EMPTY, 0.0f32)));
        assert_eq!(preceded_f32(Cbs(b"I20"), "I"), Ok((EMPTY, 20.0f32)));
        assert_eq!(preceded_f32(Cbs(b"x 1."), "X"), Ok((EMPTY, 1.0f32)));
        assert_eq!(preceded_f32(Cbs(b"x1."), "X"), Ok((EMPTY, 1.0f32)));

        assert_eq!(preceded_f32(Cbs(b"x1.23"), "X"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"y-1.23"), "Y"), Ok((EMPTY, -1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"z+1.23"), "Z"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"a123"), "A"), Ok((EMPTY, 123.0f32)));

        assert_eq!(preceded_f32(Cbs(b"X1.23"), "X"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"Y-1.23"), "Y"), Ok((EMPTY, -1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"Z+1.23"), "Z"), Ok((EMPTY, 1.23f32)));
        assert_eq!(preceded_f32(Cbs(b"A123"), "A"), Ok((EMPTY, 123.0f32)));
    }

    #[test]
    fn it_parses_preceded_unsigned_integers() {
        assert_eq!(preceded_u32(Cbs(b"x123"), "X"), Ok((EMPTY, 123u32)));
        assert_eq!(preceded_u32(Cbs(b"X123"), "X"), Ok((EMPTY, 123u32)));

        assert!(preceded_u32(Cbs(b"y-123"), "Y").is_err());
        assert!(preceded_u32(Cbs(b"Y-123"), "Y").is_err());
    }

    #[test]
    fn it_parses_preceded_codes() {
        assert_eq!(
            preceded_code(Cbs(b"G54"), 'G', 54.0),
            Ok((EMPTY, ('G', 54.0)))
        );

        assert_eq!(
            preceded_code(Cbs(b"G17.1"), 'G', 17.1),
            Ok((EMPTY, ('G', 17.1)))
        );

        assert_eq!(
            preceded_code(Cbs(b"g00"), 'g', 0.0),
            Ok((EMPTY, ('G', 0.0)))
        );
    }

    #[test]
    fn it_parses_gcodes() {
        assert_eq!(g(Cbs(b"G54"), 54.0), Ok((EMPTY, Cbs(b"G54"))));
    }

    #[test]
    fn it_parses_mcodes() {
        assert_eq!(m(Cbs(b"M30"), 30.0), Ok((EMPTY, Cbs(b"M30"))));
    }
}
