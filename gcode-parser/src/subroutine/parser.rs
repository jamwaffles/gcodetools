use super::super::tokenizer::helpers::*;
use super::super::tokenizer::{token_not_end_program, Token};
use super::{Subroutine, SubroutineName};
use nom::types::CompleteByteSlice;

named!(subroutine_name<CompleteByteSlice, SubroutineName>, map!(
    call!(preceded_u32, "O"),
    |res| SubroutineName::Number(res)
));

named!(start_sub<CompleteByteSlice, SubroutineName>,
    terminated!(subroutine_name, tag_no_case!(" sub"))
);

named_args!(end_sub(sub_name: String)<CompleteByteSlice, CompleteByteSlice>,
    terminated!(tag_no_case!(sub_name.as_str()), tag_no_case!(" endsub"))
);

named!(subroutine_definition<CompleteByteSlice, Subroutine>, ws!(
    do_parse!(
        name: start_sub >>
        tokens: terminated!(
            many0!(token_not_end_program),
            call!(end_sub, name.clone().into())
        ) >>
        ({
            Subroutine { name, tokens }
        })
    )
));

named!(subroutine_call<CompleteByteSlice, SubroutineName>,
    terminated!(subroutine_name, tag_no_case!(" call"))
);

named!(pub subroutine<CompleteByteSlice, Token>, alt_complete!(
    map!(subroutine_definition, |sub| Token::SubroutineDefinition(sub)) |
    map!(subroutine_call, |sub| Token::SubroutineCall(sub))
));

#[cfg(test)]
mod tests {
    use super::super::super::tokenizer::prelude::*;
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    macro_rules! assert_expr {
        ($to_check:expr, $against:expr) => {
            assert_eq!($to_check, Ok((EMPTY, $against)))
        };
    }

    #[test]
    fn it_parses_numbered_subroutines() {
        let input = r#"o100 sub
          G54 G0 X0 Y0 Z0
        o100 endsub"#;

        // assert_expr!(subroutine(input), vec![]);
        assert_expr!(
            subroutine(Cbs(input.as_bytes())),
            Token::SubroutineDefinition(Subroutine {
                name: SubroutineName::Number(100u32),
                tokens: vec![
                    Token::WorkOffset(WorkOffset::G54),
                    Token::RapidMove,
                    Token::Coord(Vec9 {
                        x: Some(Value::Float(0.0)),
                        y: Some(Value::Float(0.0)),
                        z: Some(Value::Float(0.0)),
                        a: None,
                        b: None,
                        c: None,
                        u: None,
                        v: None,
                        w: None,
                    }),
                ],
            })
        );
    }
}
