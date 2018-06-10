use super::super::expression::parser::expression;
use super::super::tokenizer::helpers::*;
use super::super::tokenizer::{token_not_end_program_or_subroutine, Token};
use super::{Subroutine, SubroutineCall, SubroutineName, While};
use nom::types::CompleteByteSlice;

named!(subroutine_name<CompleteByteSlice, SubroutineName>, alt_complete!(
    map!(call!(preceded_u32, "O"), |res| SubroutineName::Number(res)) |
    map!(
        flat_map!(delimited!(tag_no_case!("O<"), take_until!(">"), char!('>')), parse_to!(String)),
        |res| SubroutineName::External(res.into())
    )
));

named_args!(start_section(section_tag: String)<CompleteByteSlice, SubroutineName>,
    ws!(terminated!(subroutine_name, tag_no_case!(section_tag.as_str())))
);

named_args!(end_section(section_tag: String, sub_name: String)<CompleteByteSlice, CompleteByteSlice>,
    ws!(terminated!(tag_no_case!(sub_name.as_str()), tag_no_case!(section_tag.as_str())))
);

named!(while_definition<CompleteByteSlice, While>, ws!(
    do_parse!(
        name: call!(start_section, "while".into()) >>
        condition: expression >>
        tokens: terminated!(
            many0!(token_not_end_program_or_subroutine),
            call!(end_section, "endwhile".into(), name.clone().into())
        ) >>
        ({
            While { name, tokens, condition }
        })
    )
));

named!(subroutine_definition<CompleteByteSlice, Subroutine>, ws!(
    do_parse!(
        name: call!(start_section, "sub".into()) >>
        tokens: terminated!(
            many0!(token_not_end_program_or_subroutine),
            call!(end_section, "endsub".into(), name.clone().into())
        ) >>
        (Subroutine { name, tokens })
    )
));

named!(subroutine_call<CompleteByteSlice, SubroutineCall>, do_parse!(
    name: ws!(terminated!(subroutine_name, tag!("call"))) >>
    args: opt!(ws!(many1!(expression))) >>
    (SubroutineCall { name, args })
));

named!(pub control_flow<CompleteByteSlice, Token>, alt_complete!(
    map!(while_definition, |w| Token::While(w)) |
    map!(subroutine_call, |sub| Token::SubroutineCall(sub))
));

named!(pub subroutine<CompleteByteSlice, Token>,
    map!(subroutine_definition, |sub| Token::SubroutineDefinition(sub))
);

#[cfg(test)]
mod tests {
    use super::super::super::expression::{BinaryOperator, ExpressionToken};
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
    fn it_parses_subroutine_names() {
        assert_expr!(
            subroutine_name(Cbs(b"O100")),
            SubroutineName::Number(100u32)
        );
        assert_expr!(
            subroutine_name(Cbs(b"O<external_name>")),
            SubroutineName::External("external_name".into())
        );
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
    fn it_parses_external_subroutine_definitions() {
        let input = r#"o<external_file> sub
          G54
        o<external_file> endsub"#;

        assert_expr!(
            subroutine(Cbs(input.as_bytes())),
            Token::SubroutineDefinition(Subroutine {
                name: SubroutineName::External("external_file".into()),
                tokens: vec![Token::WorkOffset(WorkOffset::G54)],
            })
        );
    }

    #[test]
    fn it_parses_external_subroutine_calls() {
        let input = r#"o<external_file> call"#;

        assert_expr!(
            subroutine_call(Cbs(input.as_bytes())),
            SubroutineCall {
                name: SubroutineName::External("external_file".into()),
                args: None,
            }
        );
    }

    #[test]
    fn it_parses_whiles() {
        let input = r#"o100 while [ #100 le 180 ]
            g0 z0
        o100 endwhile"#;

        assert_expr!(
            control_flow(Cbs(input.as_bytes())),
            Token::While(While {
                name: SubroutineName::Number(100),
                tokens: vec![
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
                        w: None,
                    }),
                ],
                condition: vec![
                    ExpressionToken::Parameter(Parameter::Numbered(100)),
                    ExpressionToken::BinaryOperator(BinaryOperator::LessThanOrEqual),
                    ExpressionToken::Literal(180.0),
                ],
            })
        );
    }

    #[test]
    fn it_fails_on_nested_subroutines() {
        let input = r#"o100 sub
          G54 G0 X0 Y0 Z0

          o200 sub
            G54 G0 X0 Y0 Z0
          o200 endsub
        o100 endsub"#;

        assert!(subroutine(Cbs(input.as_bytes())).is_err());
    }

    #[test]
    fn it_can_call_other_subroutines() {
        let input = r#"o100 sub
          G54 G0 X0 Y0 Z0

          o200 call [10]
        o100 endsub"#;

        assert!(subroutine(Cbs(input.as_bytes())).is_ok());
    }

    #[test]
    fn it_parses_calls_with_args() {
        let input = r#"o100 call [10] [20]"#;

        assert_expr!(
            control_flow(Cbs(input.as_bytes())),
            Token::SubroutineCall(SubroutineCall {
                name: SubroutineName::Number(100u32),
                args: Some(vec![
                    vec![ExpressionToken::Literal(10.0)],
                    vec![ExpressionToken::Literal(20.0)],
                ]),
            })
        );
    }
}
