use nom::types::CompleteByteSlice;
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<CompleteByteSlice<'a>>;

#[macro_export]
macro_rules! format_parse_error {
    ($remaining:expr, $e:expr, $input:expr) => {{
        let input = String::from_utf8($input.fragment.to_vec()).unwrap();
        let total_lines = input.lines().count();
        let line_number_digits = format!("{}", total_lines).len();

        let upto = input
            .lines()
            .into_iter()
            .enumerate()
            .skip(($remaining.line as i32 - 2).max(0) as usize)
            .take(3)
            .map(|(number, line)| {
                format!(
                    "{}{:>width$} | {}",
                    if number as u32 + 1 == $remaining.line {
                        ">>> "
                    } else {
                        "    "
                    },
                    number + 1,
                    line,
                    width = line_number_digits
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            r#"Parser execution failed at {}:{}

{}
"#,
            $remaining.line,
            $remaining.get_column(),
            upto,
        )
    }};
}
