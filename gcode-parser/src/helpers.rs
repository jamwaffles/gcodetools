use nom::types::CompleteByteSlice;
use nom::*;

named!(pub take_until_line_ending<CompleteByteSlice, CompleteByteSlice>, alt_complete!(take_until!("\r\n") | take_until!("\n")));

// Parse a GCode-style float, i.e. does not support scientific notation
named!(recognize_float_no_exponent<CompleteByteSlice, CompleteByteSlice>, recognize!(
    tuple!(
        opt!(one_of!("+-")),
        alt!(
            value!((), tuple!(
                digit,
                opt!(terminated!(char!('.'), opt!(digit)))
            )) |
            value!((), tuple!(
                opt!(terminated!(opt!(digit), char!('.'))),
                digit
            ))
        )
    )
));

named!(pub float_no_exponent<CompleteByteSlice, f32>, flat_map!(
    recognize_float_no_exponent,
    parse_to!(f32)
));

named_args!(
    pub preceded_f32<'a>(preceding: &str)<CompleteByteSlice<'a>, f32>,
    ws!(preceded!(tag_no_case!(preceding), float_no_exponent))
);

named_args!(
    pub preceded_u32<'a>(preceding: &str)<CompleteByteSlice<'a>, u32>,
    flat_map!(preceded!(tag_no_case!(preceding), terminated!(digit, not!(char!('.')))), parse_to!(u32))
);

named_args!(char_no_case(search: char)<CompleteByteSlice, char>,
    alt!(char!(search.to_ascii_lowercase()) | char!(search.to_ascii_uppercase()))
);

named_args!(
    pub preceded_code_range_inclusive<'a>(preceding: char, code_low: f32, code_high: f32)<CompleteByteSlice<'a>, (char, f32)>,
    map_res!(
        preceded!(call!(char_no_case, preceding), float_no_exponent),
        |res| {
            if code_low == code_high {
                if res == code_low {
                    Ok((preceding.to_ascii_uppercase(), res))
                } else {
                    Err(())
                }
            } else if res >= code_low && res <= code_high {
                Ok((preceding.to_ascii_uppercase(), res))
            } else {
                Err(())
            }
        }
    )
);

named_args!(
    pub g<'a>(c: f32)<CompleteByteSlice<'a>, CompleteByteSlice<'a>>,
    recognize!(call!(preceded_code_range_inclusive, 'G', c, c))
);

named_args!(
    pub m<'a>(c: f32)<CompleteByteSlice<'a>, CompleteByteSlice<'a>>,
    recognize!(call!(preceded_code_range_inclusive, 'M', c, c))
);

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

        // Attempting to parse a float as a number must fail
        assert!(preceded_u32(Cbs(b"Y1.23"), "Y").is_err());
    }

    #[test]
    fn it_parses_gcodes() {
        assert_eq!(g(Cbs(b"G54"), 54.0), Ok((EMPTY, Cbs(b"G54"))));
    }

    #[test]
    fn it_parses_mcodes() {
        assert_eq!(m(Cbs(b"M30"), 30.0), Ok((EMPTY, Cbs(b"M30"))));
    }

    // Ripped from Nom 4 tests, sans test numbers with exponents
    #[test]
    fn it_parses_float_no_exponents() {
        let mut test_cases = vec![
            "+3.14", "3.14", "-3.14", "0", "0.0", "1.", ".789", "-.5", ".1",
        ];

        for test in test_cases.drain(..) {
            let expected32 = str::parse::<f32>(test).unwrap();
            let expected64 = str::parse::<f64>(test).unwrap();

            assert_eq!(
                recognize_float_no_exponent(CompleteByteSlice(test.as_bytes())),
                Ok((CompleteByteSlice(b""), CompleteByteSlice(test.as_bytes())))
            );
            let larger = format!("{};", test);
            assert_eq!(
                recognize_float_no_exponent(Cbs(larger.as_bytes())),
                Ok((Cbs(b";"), Cbs(test.as_bytes())))
            );

            assert_eq!(float(larger.as_bytes()), Ok((&b";"[..], expected32)));
            assert_eq!(float_s(&larger[..]), Ok((";", expected32)));

            assert_eq!(double(larger.as_bytes()), Ok((&b";"[..], expected64)));
            assert_eq!(double_s(&larger[..]), Ok((";", expected64)));
        }
    }

    #[test]
    fn it_parses_ranges_of_preceded_codes() {
        assert!(preceded_code_range_inclusive(Cbs(b"M100"), 'M', 100.0, 110.0).is_ok());
        assert!(preceded_code_range_inclusive(Cbs(b"M110"), 'M', 100.0, 110.0).is_ok());
        assert!(preceded_code_range_inclusive(Cbs(b"M105"), 'M', 100.0, 110.0).is_ok());
        assert!(preceded_code_range_inclusive(Cbs(b"M111"), 'M', 100.0, 110.0).is_err());
        assert!(preceded_code_range_inclusive(Cbs(b"M99"), 'M', 100.0, 110.0).is_err());

        assert_eq!(
            preceded_code_range_inclusive(Cbs(b"G54"), 'G', 54.0, 54.0),
            Ok((EMPTY, ('G', 54.0)))
        );

        assert_eq!(
            preceded_code_range_inclusive(Cbs(b"G17.1"), 'G', 17.1, 17.1),
            Ok((EMPTY, ('G', 17.1)))
        );

        assert_eq!(
            preceded_code_range_inclusive(Cbs(b"g00"), 'g', 0.0, 0.0),
            Ok((EMPTY, ('G', 0.0)))
        );
    }
}
