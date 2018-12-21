macro_rules! print_code {
    ($remaining:expr, $e:expr, $input:expr) => {{
        let remaining = String::from_utf8($remaining.fragment.to_vec()).unwrap();
        let input = String::from_utf8($input.fragment.to_vec()).unwrap();

        panic!(
            "Parser execution failed\n-- Test input is (len {})\n{}\n\n-- Error type\n{:?}\n\n-- Remaining input is (len {})\n{}\n",
            input.len(),
            input,
            $e,
            remaining.len(),
            remaining
        );
    }}
}

#[macro_export]
macro_rules! assert_parse {
    (parser = $parser:expr, input = $input:expr, expected = $compare:expr) => {
        use crate::Span;
        use nom::types::CompleteByteSlice;

        match $parser($input) {
            Ok(result) => assert_eq!(result, (Span::new(CompleteByteSlice(b"")), $compare)),
            Err(Err::Error(Context::Code(remaining, e))) => print_code!(remaining, e, $input),
            Err(e) => panic!("Parse execution failed: {:?}", e),
        }
    };

    (parser = $parser:expr, input = $input:expr, expected = $compare:expr, remaining = $remaining:expr) => {
        match $parser($input) {
            Ok(result) => assert_eq!(result, ($remaining, $compare)),
            Err(Err::Error(Context::Code(remaining, e))) => print_code!(remaining, e, $input),
            Err(e) => panic!("Parse execution failed: {:?}", e),
        }
    };
}

#[macro_export]
macro_rules! coord {
    ($span:expr, $x:expr, $y:expr, $z:expr) => {
        Coord {
            span: $span,
            x: Some($x),
            y: Some($y),
            z: Some($z),
            ..Coord::default()
        }
    }; // TODO: Other permutations of args
}

#[macro_export]
macro_rules! span {
    ($content:expr, offset = $offset:expr, line = $line:expr) => {{
        use nom::types::CompleteByteSlice;

        Span {
            offset: $offset,
            line: $line,
            fragment: CompleteByteSlice($content),
        }
    }};
    ($content:expr, offset = $offset:expr) => {{
        use nom::types::CompleteByteSlice;

        Span {
            offset: $offset,
            line: 1,
            fragment: CompleteByteSlice($content),
        }
    }};
    ($content:expr) => {{
        use nom::types::CompleteByteSlice;

        Span::new(CompleteByteSlice($content))
    }};
}

#[macro_export]
macro_rules! empty_span {
    (offset = $offset:expr, line = $line:expr) => {{
        use nom::types::CompleteByteSlice;

        Span {
            offset: $offset,
            line: $line,
            fragment: CompleteByteSlice(b""),
        }
    }};

    (offset = $offset:expr) => {{
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
