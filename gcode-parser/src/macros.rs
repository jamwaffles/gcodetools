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
    });
    ($func:expr, $i:expr, $num:expr) => ({
        use nom::*;

        match $func {
            Ok((remaining, num)) => if num == $num {
                Ok((remaining, num))
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
        use $crate::helpers::preceded_u32;

        map_result!(preceded_u32($i, "G"), $i, $num, $mapto)
    });
    ($i:expr, $num:expr) => ({
        use $crate::helpers::preceded_u32;

        map_result!(preceded_u32($i, "G"), $i, $num)
    });
);

#[macro_export]
macro_rules! g_float(
    ($i:expr, $num:expr, $mapto:expr) => ({
        use $crate::helpers::preceded_f32;

        map_result!(preceded_f32($i, "G"), $i, $num, $mapto)
    });
    ($i:expr, $num:expr) => ({
        use $crate::helpers::preceded_f32;

        map_result!(preceded_f32($i, "G"), $i, $num)
    });
);

#[macro_export]
macro_rules! m_int(
    ($i:expr, $num:expr, $mapto:expr) => ({
        use $crate::helpers::preceded_u32;

        map_result!(preceded_u32($i, "M"), $i, $num, $mapto)
    });
    ($i:expr, $num:expr) => ({
        use $crate::helpers::preceded_u32;

        map_result!(preceded_u32($i, "M"), $i, $num)
    });
);
