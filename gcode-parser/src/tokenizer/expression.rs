use nom::types::CompleteByteSlice;
use nom::*;

#[cfg(test)]
mod tests {
    use super::*;
    use nom;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    #[test]
    fn it_assigns_numbers_to_numbered_parameters() {
        let input = Cbs(b"#1234 = 56");
    }

    fn it_assigns_numbers_to_named_parameters() {
        let input = Cbs(b"#<named_param> = 56");
    }

    fn it_assigns_numbers_to_global_parameters() {
        let input = Cbs(b"#<_global_param> = 56");
    }

    fn it_parses_expressions() {
        let input = Cbs(b"[1 + 2]");
    }

    fn it_parses_expressions_with_numbered_parameters() {
        let input = Cbs(b"#1234 = 56\n[1 + 2 + #1234]");
    }

    fn it_assigns_expressions_to_parameters() {
        let input_numbered = Cbs(b"#1234 = [1 + 2]");
        let input_named = Cbs(b"#<named_param> = [1 + 2]");
        let input_global = Cbs(b"#<_global_param> = [1 + 2]");
    }
}
