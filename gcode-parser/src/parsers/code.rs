#[macro_export]
macro_rules! code(
    ($i:expr, $code:expr, $following:ident!( $($args:tt)* )) => ({
        sep!(
            $i,
            space0,
            preceded!(
                code!($code),
                $following!($($args)*)
            )
        )
    });
    ($i:expr, $code:expr, $following:expr) => (
        code!($i, call!($code, $following));
    );
    ($i:expr, $code:expr) => ({
        use nom::*;

        let (letter, number) = $code.split_at(1);

        preceded!(
            $i,
            tag_no_case!(letter),
            alt_complete!(
                delimited!(
                    char!('0'),
                    tag!(number),
                    not!(one_of!(".1234567890"))
                ) |
                terminated!(
                    tag!(number),
                    not!(one_of!(".1234567890"))
                )
            )

        )
    });
);

#[macro_export]
macro_rules! map_code(
    ($i:expr, $code:expr, $map:expr) => ({
        use $crate::code;

        map!(
            $i,
            code!($code),
            $map
        )
    });

    ($i:expr, $code:expr, $following:ident!( $($args:tt)* ), $map:expr) => ({
        use $crate::code;

        map!(
            $i,
            code!($code, $following!($($args)*)),
            $map
        )
    });
);

#[cfg(test)]
mod tests {
    #[test]
    fn parse_integer_code() {
        let out = code!(span!(b"G54"), "G54");

        assert_eq!(out, Ok((empty_span!(offset = 3), span!(b"54", offset = 1))));
    }

    #[test]
    fn parse_single_integer_code() {
        let out = code!(span!(b"G0"), "G0");

        assert_eq!(out, Ok((empty_span!(offset = 2), span!(b"0", offset = 1))));
    }

    #[test]
    fn parse_decimal_code() {
        let out = code!(span!(b"G17.1"), "G17.1");

        assert_eq!(
            out,
            Ok((empty_span!(offset = 5), span!(b"17.1", offset = 1)))
        );
    }

    #[test]
    fn decimal_strict_match() {
        let out = code!(span!(b"G17.1"), "G17");

        assert!(out.is_err());
    }

    #[test]
    fn parse_leading_zeros() {
        let out = code!(span!(b"G01"), "G1");

        assert_eq!(out, Ok((empty_span!(offset = 3), span!(b"1", offset = 2))));
    }

    #[test]
    fn ignore_trailing_other_chars() {
        assert_eq!(
            code!(span!(b"G17.1 G54"), "G17.1"),
            Ok((span!(b" G54", offset = 5), span!(b"17.1", offset = 1)))
        );

        assert_eq!(
            code!(span!(b"G17.1G54"), "G17.1"),
            Ok((span!(b"G54", offset = 5), span!(b"17.1", offset = 1)))
        );
    }
}
