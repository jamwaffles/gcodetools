macro_rules! map_code_result(
    ($func:expr, $mapto:expr) => ({
        match $func {
            Ok((remaining, _)) => Ok((remaining, $mapto)),
            Err(args) => Err(args)
        }
    });
);

#[macro_export]
macro_rules! g_code(
    ($i:expr, $num:expr, $mapto:expr) => ({
        use $crate::helpers::code;

        map_code_result!(code($i, "G", $num), $mapto)
    });
    ($i:expr, $num:expr) => ({
        use $crate::helpers::code;

        code($i, "G", $num)
    });
);

#[macro_export]
macro_rules! m_code(
    ($i:expr, $num:expr, $mapto:expr) => ({
        use $crate::helpers::code;

        map_code_result!(code($i, "M", $num), $mapto)
    });
    ($i:expr, $num:expr) => ({
        use $crate::helpers::code;

        code($i, "M", $num)
    });
);

#[cfg(test)]
#[macro_export]
macro_rules! assert_complete_parse {
    ($to_check:expr, $against:expr) => {
        assert_eq!($to_check, Ok((CompleteByteSlice(b""), $against)))
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_remaining {
    ($to_check:expr) => {
        match $to_check {
            Ok((rem, _)) => assert!(rem.len() > 0),
            _ => assert!(false),
        }
    };
}
