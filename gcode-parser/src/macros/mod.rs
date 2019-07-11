/// Create a coordinate with only some fields populated
#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr, $z:expr) => {
        Coord {
            x: Some(($x).into()),
            y: Some(($y).into()),
            z: Some(($z).into()),
            ..Coord::default()
        }
    };
    ($x:expr, $y:expr) => {
        Coord {
            x: Some(($x).into()),
            y: Some(($y).into()),
            ..Coord::default()
        }
    };
}

/// Check a test result
#[macro_export]
macro_rules! assert_parse {
    (parser = $parser:ident; input = $($input:expr),+; expected = $($expected:expr),+ $(;)*) => {
        let inputs = vec![$($input),+];
        let comparisons = vec![$($expected),+];

        for (input, expected) in inputs.into_iter().zip(comparisons.into_iter()) {
            let res = $parser(input);

            let res = res.map_err(|e| match e {
                nom::Err::Error(e) | nom::Err::Failure(e) => {
                    nom::error::convert_error(input, e)
                }
                e => format!("Failed to parse input `{}' for reason: {:?}", input, e),
            });

            match res {
                Ok((remaining, result)) => {
                    assert_eq!(remaining.len(), 0, "{} bytes remaining to consume: \"{}\"", remaining.len(), remaining);
                    assert_eq!(result, expected);
                },
                Err(e) => panic!("{}", e)
            }
        }
    };

    (parser = $parser:ident( $($parse_args:tt)* ); expected = $expected:expr $(;)*) => {
        match $parser($($parse_args)*) {
            Ok(result) => assert_eq!(
                result.1,
                $expected
            ),
            Err(nom::Err::Error(nom::Context::Code(_remaining, e))) => {
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
            let res = $parser(input).map_err(|e| match e {
                nom::Err::Error(e) | nom::Err::Failure(e) => {
                    nom::error::convert_error(input, e)
                }
                e => format!("Failed to parse input `{}' for reason: {:?}. Remaining: `{}'", input, e, remaining),
            });

            match res {
                Ok((result_remaining, result)) => {
                    assert_eq!(result_remaining, remaining);
                    assert_eq!(result, expected);
                },
                Err(e) => panic!("{}", e)
            }
        }
    };
}
