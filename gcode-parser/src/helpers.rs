#[cfg(test)]
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

#[cfg(test)]
#[macro_export]
macro_rules! assert_parse {
    (parser = $parser:expr, input = $input:expr, expected = $compare:expr) => {
        use crate::Span;

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

#[cfg(test)]
#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr, $z:expr) => {
        Coord {
            x: Some($x),
            y: Some($y),
            z: Some($z),
            ..Coord::default()
        }
    }; // TODO: Other permutations of args
}

#[cfg(test)]
#[macro_export]
macro_rules! span {
    ($content:expr, offset = $offset:expr, line = $line:expr) => {
        Span {
            offset: $offset,
            line: $line,
            fragment: CompleteByteSlice($content),
        }
    };
    ($content:expr, offset = $offset:expr) => {
        Span {
            offset: $offset,
            line: 1,
            fragment: CompleteByteSlice($content),
        }
    };
    ($content:expr) => {
        Span::new(CompleteByteSlice($content))
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! empty_span {
    (offset = $offset:expr, line = $line:expr) => {
        Span {
            offset: $offset,
            line: $line,
            fragment: CompleteByteSlice(b""),
        }
    };

    (offset = $offset:expr) => {
        Span {
            offset: $offset,
            line: 1,
            fragment: CompleteByteSlice(b""),
        }
    };
    () => {
        Span::new(CompleteByteSlice(b""))
    };
}
