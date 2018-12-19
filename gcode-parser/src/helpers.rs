#[cfg(test)]
macro_rules! print_code {
    ($remaining:expr, $e:expr, $input:expr) => {{
        let remaining = String::from_utf8($remaining.to_vec()).unwrap();
        let input = String::from_utf8($input.to_vec()).unwrap();

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
    ($parser:expr, $input:expr, $compare:expr) => {
        match $parser($input) {
            Ok(result) => assert_eq!(result, (CompleteByteSlice(b""), $compare)),
            Err(Err::Error(Context::Code(remaining, e))) => print_code!(remaining, e, $input),
            Err(e) => panic!("Parse execution failed: {:?}", e),
        }
    };

    ($parser:expr, $input:expr, $compare:expr, $remaining:expr) => {
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
    };
}
