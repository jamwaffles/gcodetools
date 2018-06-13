#[macro_export]
macro_rules! g_int(
    ($i:expr, $num:expr, $mapto:expr) => ({
        use nom::*;

        match preceded_u32($i, "G") {
            Ok((remaining, num)) => if num == $num {
                Ok((remaining, $mapto))
            } else {
                Err(Err::Error(Context::Code($i, ErrorKind::Digit::<u32>)))
            },
            Err(args) => Err(args)
        }
    });
);
