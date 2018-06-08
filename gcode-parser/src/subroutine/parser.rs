use super::super::expression::parser::expression;
use super::super::tokenizer::helpers::*;
use super::super::tokenizer::{token_not_end_program, Token};
use super::{Subroutine, SubroutineName, While};
use nom::types::CompleteByteSlice;

named!(subroutine_name<CompleteByteSlice, SubroutineName>, map!(
    call!(preceded_u32, "O"),
    |res| SubroutineName::Number(res)
));

named_args!(start_section(section_tag: String)<CompleteByteSlice, SubroutineName>,
    terminated!(subroutine_name, tag_no_case!(section_tag.as_str()))
);

named_args!(end_section(section_tag: String, sub_name: String)<CompleteByteSlice, CompleteByteSlice>,
    terminated!(tag_no_case!(sub_name.as_str()), tag_no_case!(section_tag.as_str()))
);

named!(while_definition<CompleteByteSlice, While>, ws!(
    do_parse!(
        name: call!(start_section, " while".into()) >>
        condition: expression >>
        tokens: terminated!(
            many0!(token_not_end_program),
            call!(end_section, " endwhile".into(), name.clone().into())
        ) >>
        ({
            While { name, tokens, condition }
        })
    )
));

named!(subroutine_definition<CompleteByteSlice, Subroutine>, ws!(
    do_parse!(
        name: call!(start_section, " sub".into()) >>
        tokens: terminated!(
            many0!(token_not_end_program),
            call!(end_section, " endsub".into(), name.clone().into())
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
    map!(while_definition, |w| Token::While(w)) |
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

    #[test]
    fn it_parses_whiles() {
        let input = r#"o100 while [ #100 le 180 ]
            g0 z0
        o100 endwhile"#;

        assert_expr!(
            subroutine(Cbs(input.as_bytes())),
            Token::While(While {
                name: SubroutineName::Number(100),
                tokens: [
                    Token::RapidMove,
                    Token::Coord(Vec9 {
                        x: None,
                        y: None,
                        z: Some(Value::Float(0.0)),
                        a: None,
                        b: None,
                        c: None,
                        u: None,
                        v: None,
                        w: None
                    }),
                ],
                condition: [
                    Parameter(Numbered(100)),
                    BinaryOperator(LessThanOrEqual),
                    Literal(180.0),
                ]
            })
        );
    }
}
