#[cfg(test)]
#[macro_use]
mod test_helpers;

macro_rules! snip {
    ($input:expr, $n:expr) => {{
        let lines = $input.lines();
        let num_lines = $input.lines().count();

        if num_lines > $n {
            // TODO: Fix clone
            let start = lines.clone().take($n / 2).collect::<Vec<&str>>().join("\n");
            let mut end = lines.rev().take($n / 2).collect::<Vec<&str>>();

            end.reverse();

            let end = end.join("\n");

            format!("{}\n...\n{}", start, end)
        } else {
            lines.take($n).collect::<Vec<&str>>().join("\n")
        }
    }};

    ($input:expr) => {
        snip!($input, 10)
    };
}

#[macro_export]
macro_rules! format_parse_error {
    ($remaining:expr, $e:expr, $input:expr) => {{
        let remaining = String::from_utf8($remaining.fragment.to_vec()).unwrap();
        let input = String::from_utf8($input.fragment.to_vec()).unwrap();

        format!(
            "Parser execution failed\n-- Test input (len {})\n{}\n\n-- Error type\n{:?}\n\n-- Remaining input (len {})\n{}\n",
            input.len(),
            snip!(input),
            $e,
            remaining.len(),
            snip!(remaining)
        )
    }}
}

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
    ($i:expr, $code:expr) => ({
        tag_no_case!($i, $code);
    });
);

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
