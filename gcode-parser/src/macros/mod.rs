#[cfg(test)]
#[macro_use]
mod test_helpers;

// macro_rules! snip {
//     ($input:expr, $n:expr) => {{
//         let lines = $input.lines();
//         let num_lines = $input.lines().count();

//         if num_lines > $n {
//             // TODO: Fix clone
//             let start = lines.clone().take($n / 2).collect::<Vec<&str>>().join("\n");
//             let mut end = lines.rev().take($n / 2).collect::<Vec<&str>>();

//             end.reverse();

//             let end = end.join("\n");

//             format!("{}\n...\n{}", start, end)
//         } else {
//             lines.take($n).collect::<Vec<&str>>().join("\n")
//         }
//     }};

//     ($input:expr) => {
//         snip!($input, 10)
//     };
// }

#[macro_export]
macro_rules! format_parse_error {
    ($remaining:expr, $e:expr, $input:expr) => {{
        let remaining = String::from_utf8($remaining.fragment.to_vec()).unwrap();
        let input = String::from_utf8($input.fragment.to_vec()).unwrap();

        let last_good_line = input.lines().last().unwrap();
        let erroring_line = remaining.lines().next().unwrap();

        format!(
            r#"Parser execution failed at {}:{}

{}
{}
{}╯"#,
            $remaining.line,
            $remaining.get_column(),
            last_good_line,
            erroring_line,
            (0..$remaining.get_column() - 1)
                .map(|_| "─")
                .collect::<String>()
        )
    }};
}

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

// TODO: Move to parsers/
#[macro_export]
macro_rules! line_with (
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        terminated!(
            $i,
            $submac!($($args)*),
            alt_complete!(line_ending | eof!())
        )
    });
    ($i:expr, $submac:expr) => (
        line_with!($i, call!($submac));
    );
);
