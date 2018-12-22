#[cfg(test)]
#[macro_use]
mod test_helpers;

#[macro_export]
macro_rules! code(
    ($i:expr, $code:expr, $following:ident!( $($args:tt)* )) => ({
        sep!(
            $i,
            space0,
            preceded!(
                tag_no_case!($code),
                $following!($($args)*)
            )
        )
    });
    ($i:expr, $code:expr, $following:expr) => (
        code!($i, call!($code, $following));
    );
);

#[macro_export]
macro_rules! positioned(
    ($i:expr, $submac:ident!( $($args:tt)* ), $map:expr) => ({
        map!(
        	$i,
            tuple!(
                position!(),
                $submac!($($args)*)
            ),
            $map
        )
    });
    ($i:expr, $submac:expr) => (
        code!($i, call!($code, $following));
    );
);
