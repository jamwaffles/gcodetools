#[cfg(test)]
#[macro_export]
macro_rules! assert_complete_parse {
    ($to_check:expr, $against:expr) => {
        assert_eq!(
            $to_check,
            Ok((
                $crate::empty_span!(offset = $to_check.fragment.len()),
                $against
            ))
        )
    };
}

// TODO: Export from common helpers crate. This macro is duplicated in the gcode-parser crate
#[cfg(test)]
#[macro_export]
macro_rules! span {
    ($content:expr, offset = $offset:expr, line = $line:expr) => {{
        use gcode_parser::Span;
        use nom::types::CompleteByteSlice;

        Span {
            offset: $offset,
            line: $line,
            fragment: CompleteByteSlice($content),
        }
    }};
    ($content:expr, offset = $offset:expr) => {{
        use gcode_parser::Span;
        use nom::types::CompleteByteSlice;

        Span {
            offset: $offset,
            line: 1,
            fragment: CompleteByteSlice($content),
        }
    }};
    ($content:expr) => {{
        use gcode_parser::Span;
        use nom::types::CompleteByteSlice;

        Span::new(CompleteByteSlice($content))
    }};
}

// TODO: Condense into span!() above
#[cfg(test)]
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
