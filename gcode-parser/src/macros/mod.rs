#[cfg(test)]
#[macro_use]
mod test_helpers;

/// Wrap a parser to return a start position with it as a tuple of `(position, result)`
// TODO: Move to parsers/
#[macro_export]
macro_rules! positioned(
    ($i:expr, $submac:ident!( $($args:tt)* ), $map:expr) => ({
        use nom_locate::position;
        use nom::{map, tuple};

        map!(
            $i,
            tuple!(
                position!(),
                $submac!($($args)*)
            ),
            $map
        )
    });
    ($i:expr, $f:expr) => (
        opt!($i, call!($f));
    );
);

/// As `positioned!()` but allows returning the parse result in a `Result`
// TODO: Move to parsers/
#[macro_export]
macro_rules! positioned_res(
    ($i:expr, $submac:ident!( $($args:tt)* ), $map:expr) => ({
        map_res!(
            $i,
            tuple!(
                position!(),
                $submac!($($args)*)
            ),
            $map
        )
    });
    ($i:expr, $submac:expr) => (
        positioned_res!($i, call!($code, $following));
    );
);

/// Match a single character using `.eq_ignore_ascii_case()`
// TODO: Move to parsers/
#[macro_export]
macro_rules! char_no_case (
    ($i:expr, $c: expr) => ({
        use nom::Slice;
        use nom::AsChar;
        use nom::InputIter;
        use nom::Err;
        use nom::ErrorKind;
        use nom::Context;
        use nom::need_more;

        match ($i).iter_elements().next().map(|c| {
            c.as_char().eq_ignore_ascii_case(&$c)
        }) {
            None        => need_more($i, Needed::Size(1)),
            Some(false) => {
                let e: ErrorKind<u32> = nom::ErrorKind::Char;

                Err(Err::Error(Context::Code($i, e)))
            },
            Some(true)  => Ok(( $i.slice($c.len()..), $i.iter_elements().next().unwrap().as_char() ))
        }
    });
);

/// Parse a single line with a given parser
// TODO: Move to parsers/
#[macro_export]
macro_rules! line_with (
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        terminated!(
            $i,
            $submac!($($args)*),
            alt!(line_ending | eof!())
        )
    });
    ($i:expr, $submac:expr) => (
        line_with!($i, call!($submac));
    );
);
