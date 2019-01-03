#[cfg(test)]
#[macro_export]
macro_rules! assert_parse {
    (parser = $parser:ident; input = $($input:expr),+; expected = $($expected:expr),+ $(;)*) => {
        use nom::*;

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
                Err(Err::Error(Context::Code(_remaining, e))) => {
                    panic!("Parse failed: {:?}", e)
                }
                Err(e) => panic!("Parse execution failed: {:?}", e),
            }
        }
    };

    (parser = $parser:ident( $($parse_args:tt)* ); expected = $expected:expr $(;)*) => {
        match $parser($($parse_args)*) {
            Ok(result) => assert_eq!(
                result.1,
                $expected
            ),
            Err(Err::Error(Context::Code(_remaining, e))) => {
                panic!("Parse failed: {:?}", e)
            }
            Err(e) => panic!("Parse execution failed: {:?}", e),
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
                Err(Err::Error(Context::Code(_remaining, e))) => {
                    panic!("Parse failed: {:?}", e)
                }
                Err(e) => panic!("Parse execution failed: {:?}", e),
            }
        }
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_parse_ok {
    (parser = $parser:expr, input = $input:expr) => {
        assert!($parser($input).is_ok());
    };
}

// TODO: Export from common helpers crate. This macro is duplicated in the gcode-parser crate
#[cfg(test)]
#[macro_export]
macro_rules! span {
    ($content:expr, offset = $offset:expr, line = $line:expr) => {{
        use common::parsing::Span;
        use nom::types::CompleteByteSlice;

        Span {
            offset: $offset,
            line: $line,
            fragment: CompleteByteSlice($content),
        }
    }};
    ($content:expr, offset = $offset:expr) => {{
        use common::parsing::Span;
        use nom::types::CompleteByteSlice;

        Span {
            offset: $offset,
            line: 1,
            fragment: CompleteByteSlice($content),
        }
    }};
    ($content:expr) => {{
        use common::parsing::Span;
        use nom::types::CompleteByteSlice;

        Span::new(CompleteByteSlice($content))
    }};
}

// TODO: Condense into span!() above
#[cfg(test)]
#[macro_export]
macro_rules! empty_span {
    (offset = $offset:expr, line = $line:expr) => {{
        use common::parsing::Span;
        use nom::types::CompleteByteSlice;

        Span {
            offset: $offset,
            line: $line,
            fragment: CompleteByteSlice(b""),
        }
    }};

    (offset = $offset:expr) => {{
        use common::parsing::Span;
        use nom::types::CompleteByteSlice;

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
