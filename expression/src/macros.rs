#[cfg(test)]
#[macro_export]
macro_rules! assert_complete_parse {
    ($to_check:expr, $against:expr) => {
        assert_eq!($to_check, Ok((CompleteByteSlice(b""), $against)))
    };
}
