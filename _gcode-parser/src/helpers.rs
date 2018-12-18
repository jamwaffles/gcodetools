use nom::types::CompleteByteSlice;
use nom::*;

named!(pub take_until_line_ending<CompleteByteSlice, CompleteByteSlice>, alt_complete!(take_until!("\r\n") | take_until!("\n")));
named!(pub take_until_line_ending_and_consumer<CompleteByteSlice, CompleteByteSlice>, alt_complete!(take_until_and_consume!("\r\n") | take_until_and_consume!("\n")));

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
    sep!(space0, preceded!(tag_no_case!(preceding), float_no_exponent))
);

named_args!(
    pub recognize_preceded_u32<'a>(preceding: &str)<CompleteByteSlice<'a>, CompleteByteSlice<'a>>,
    sep!(space0, preceded!(tag_no_case!(preceding), terminated!(digit, not!(char!('.')))))
);

named_args!(
    pub preceded_u32<'a>(preceding: &str)<CompleteByteSlice<'a>, u32>,
    flat_map!(call!(recognize_preceded_u32, preceding), parse_to!(u32))
);

named_args!(
    pub preceded_code_range_inclusive<'a>(preceding: &str, code_low: u32, code_high: u32)<CompleteByteSlice<'a>, u32>,
    map_res!(
        call!(preceded_u32, preceding),
        |res| {
            if res >= code_low && res <= code_high {
                Ok(res)
            } else {
                Err(())
            }
        }
    )
);

named_args!(
    pub code<'a>(preceding: &str, code: &str)<CompleteByteSlice<'a>, CompleteByteSlice<'a>>,
    preceded!(
        tag_no_case!(preceding),
        terminated!(
            alt!(
                preceded!(char!('0'), tag!(code)) |
                tag!(code)
            ),
            not!(one_of!(".1234567890"))
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

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
        assert_complete_parse!(preceded_f32(Cbs(b"J0"), "J"), 0.0f32);
        assert_complete_parse!(preceded_f32(Cbs(b"I20"), "I"), 20.0f32);
        assert_complete_parse!(preceded_f32(Cbs(b"x 1."), "X"), 1.0f32);
        assert_complete_parse!(preceded_f32(Cbs(b"x1."), "X"), 1.0f32);

        assert_complete_parse!(preceded_f32(Cbs(b"x1.23"), "X"), 1.23f32);
        assert_complete_parse!(preceded_f32(Cbs(b"y-1.23"), "Y"), -1.23f32);
        assert_complete_parse!(preceded_f32(Cbs(b"z+1.23"), "Z"), 1.23f32);
        assert_complete_parse!(preceded_f32(Cbs(b"a123"), "A"), 123.0f32);

        assert_complete_parse!(preceded_f32(Cbs(b"X1.23"), "X"), 1.23f32);
        assert_complete_parse!(preceded_f32(Cbs(b"Y-1.23"), "Y"), -1.23f32);
        assert_complete_parse!(preceded_f32(Cbs(b"Z+1.23"), "Z"), 1.23f32);
        assert_complete_parse!(preceded_f32(Cbs(b"A123"), "A"), 123.0f32);
    }

    #[test]
    fn it_recognizes_preceded_codes() {
        assert_complete_parse!(code(Cbs(b"G00"), "G", "0"), Cbs(b"0"));
        assert_complete_parse!(code(Cbs(b"G01"), "G", "1"), Cbs(b"1"));
        assert_complete_parse!(code(Cbs(b"G1"), "G", "1"), Cbs(b"1"));
        assert_complete_parse!(code(Cbs(b"G10"), "G", "10"), Cbs(b"10"));
        assert_complete_parse!(code(Cbs(b"G38.2"), "G", "38.2"), Cbs(b"38.2"));
        assert_complete_parse!(code(Cbs(b"G038.2"), "G", "38.2"), Cbs(b"38.2"));

        assert!(code(Cbs(b"G10"), "G", "10.1").is_err());
        assert!(code(Cbs(b"G10.5"), "G", "10.6").is_err());
    }

    #[test]
    fn it_parses_preceded_unsigned_integers() {
        assert_complete_parse!(preceded_u32(Cbs(b"x123"), "X"), 123u32);
        assert_complete_parse!(preceded_u32(Cbs(b"X123"), "X"), 123u32);
        assert_complete_parse!(preceded_u32(Cbs(b"y 123"), "Y"), 123u32);
        assert_complete_parse!(preceded_u32(Cbs(b"y 123"), "Y"), 123u32);
        assert_complete_parse!(preceded_u32(Cbs(b"G00"), "G"), 0u32);
        assert_complete_parse!(preceded_u32(Cbs(b"G01"), "G"), 1u32);
        assert_complete_parse!(preceded_u32(Cbs(b"G1"), "G"), 1u32);

        assert!(preceded_u32(Cbs(b"y-123"), "Y").is_err());
        assert!(preceded_u32(Cbs(b"Y-123"), "Y").is_err());

        // Attempting to parse a float as a number must fail
        assert!(preceded_u32(Cbs(b"Y1.23"), "Y").is_err());
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
                recognize_float_no_exponent(Cbs(test.as_bytes())),
                Ok((Cbs(b""), Cbs(test.as_bytes())))
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
}
