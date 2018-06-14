macro_rules! map_result(
    ($func:expr, $i:expr, $num:expr, $mapto:expr) => ({
        use nom::*;

        match $func {
            Ok((remaining, num)) => if num == $num {
                Ok((remaining, $mapto))
            } else {
                Err(Err::Error(Context::Code($i, ErrorKind::Digit::<u32>)))
            },
            Err(args) => Err(args)
        }
    })
);

#[macro_export]
macro_rules! g_int(
    ($i:expr, $num:expr, $mapto:expr) => ({
        use $crate::tokenizer::helpers::preceded_u32;

        map_result!(preceded_u32($i, "G"), $i, $num, $mapto)
    });
);

#[macro_export]
macro_rules! g_float(
    ($i:expr, $num:expr, $mapto:expr) => ({
        use $crate::tokenizer::helpers::preceded_f32;

        map_result!(preceded_f32($i, "G"), $i, $num, $mapto)
    });
);

#[macro_export]
macro_rules! m_int(
    ($i:expr, $num:expr, $mapto:expr) => ({
        use $crate::tokenizer::helpers::preceded_u32;

        map_result!(preceded_u32($i, "M"), $i, $num, $mapto)
    });
);

#[macro_export]
macro_rules! m_float(
    ($i:expr, $num:expr, $mapto:expr) => ({
        use $crate::tokenizer::helpers::preceded_f32;

        map_result!(preceded_f32($i, "M"), $i, $num, $mapto)
    });
);
