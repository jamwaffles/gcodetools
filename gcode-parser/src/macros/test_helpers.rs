#[macro_export]
macro_rules! assert_parse {
    (parser = $parser:ident; input = $($input:expr),+; expected = $($expected:expr),+ $(;)*) => {
        let inputs = vec![$($input),+];
        let comparisons = vec![$($expected),+];

        for (input, expected) in inputs.into_iter().zip(comparisons.into_iter()) {
            match $parser(input) {
                Ok(result) => assert_eq!(
                    result,
                    (
                        empty_span!(offset = input.fragment.len()),
                        expected
                    )
                ),
                Err(Err::Error(Context::Code(remaining, _e))) => {
                    panic!(format_parse_error!(remaining, e, input))
                }
                Err(e) => panic!("Parse execution failed: {:?}", e),
            }
        }
    };

    (parser = $parser:ident; input = $($input:expr),+; expected = $($expected:expr),+; remaining = $($remaining:expr),+ $(;)*) => {
        let inputs = vec![$($input),+];
        let comparisons = vec![$($expected),+];
        let remaining = vec![$($remaining),+];

        for ((input, expected), remaining) in inputs.into_iter().zip(comparisons.into_iter()).zip(remaining.into_iter()) {
            match $parser(input) {
                Ok(result) => assert_eq!(
                    result,
                    (
                        remaining,
                        expected
                    )
                ),
                Err(Err::Error(Context::Code(remaining, _e))) => {
                    panic!(format_parse_error!(remaining, e, input))
                }
                Err(e) => panic!("Parse execution failed: {:?}", e),
            }
        }
    };
}

#[macro_export]
macro_rules! assert_parse_ok {
    (parser = $parser:expr, input = $input:expr) => {
        assert!($parser($input).is_ok());
    };
}

#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr, $z:expr) => {
        Coord {
            x: Some($x),
            y: Some($y),
            z: Some($z),
            ..Coord::default()
        }
    };
    ($x:expr, $y:expr) => {
        Coord {
            x: Some($x),
            y: Some($y),
            ..Coord::default()
        }
    }; // TODO: Other permutations of args
}

#[macro_export]
macro_rules! span {
    ($content:expr, offset = $offset:expr, line = $line:expr) => {{
        use nom::types::CompleteByteSlice;
        use $crate::Span;

        Span {
            offset: $offset,
            line: $line,
            fragment: CompleteByteSlice($content),
        }
    }};
    ($content:expr, offset = $offset:expr) => {{
        use nom::types::CompleteByteSlice;
        use $crate::Span;

        Span {
            offset: $offset,
            line: 1,
            fragment: CompleteByteSlice($content),
        }
    }};
    ($content:expr) => {{
        use nom::types::CompleteByteSlice;
        use $crate::Span;

        Span::new(CompleteByteSlice($content))
    }};
}

#[macro_export]
macro_rules! empty_span {
    (offset = $offset:expr, line = $line:expr) => {{
        use nom::types::CompleteByteSlice;
        use $crate::Span;

        Span {
            offset: $offset,
            line: $line,
            fragment: CompleteByteSlice(b""),
        }
    }};

    (offset = $offset:expr) => {{
        use nom::types::CompleteByteSlice;
        use $crate::Span;

        Span {
            offset: $offset,
            line: 1,
            fragment: CompleteByteSlice(b""),
        }
    }};
    () => {{
        use nom::types::CompleteByteSlice;

        Span::new(CompleteByteSlice(b""))
    }};
}
