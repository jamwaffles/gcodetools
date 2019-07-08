use crate::token::block::{block_ident, BlockIdent};
use expression::{parser::gcode, Expression};
use nom::{
    bytes::complete::tag_no_case,
    character::complete::space1,
    combinator::map,
    error::{context, ParseError},
    multi::many0,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::str::FromStr;

/// Which type of block this is
#[derive(Debug, PartialEq, Clone)]
pub struct Call<T> {
    subroutine_ident: BlockIdent,
    arguments: Vec<Expression<T>>,
}

pub fn call<'a, E: ParseError<&'a str>, T>(i: &'a str) -> IResult<&'a str, Call<T>, E>
where
    T: FromStr,
{
    context(
        "subroutine call",
        map(
            separated_pair(
                block_ident,
                preceded(space1, tag_no_case("call")),
                many0(preceded(space1, gcode::expression)),
            ),
            |(subroutine_ident, arguments)| Call {
                subroutine_ident,
                arguments,
            },
        ),
    )(i)
}

// named!(pub call<Span, Call>,
//     sep!(
//         space0,
//         do_parse!(
//             subroutine_ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
//             tag_no_case!("call") >>
//             arguments: many0!(gcode_expression) >>
//             ({
//                 Call {
//                     subroutine_ident,
//                     arguments
//                 }
//             })
//         )
//     )
// );

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use expression::{ArithmeticOperator, Expression, ExpressionToken};

    #[test]
    fn parse_call_no_args() {
        let expd: Call<f32> = Call {
            subroutine_ident: "o100".into(),
            arguments: Vec::new(),
        };

        assert_parse!(
            parser = call;
            input = "o100 call";
            expected = expd;
        );
    }

    #[test]
    fn parse_call() {
        assert_parse!(
            parser = call;
            input = "o100 call [100] [1 + 2]";
            expected = Call {
                subroutine_ident: "o100".into(),
                arguments: vec![
                    Expression::from_tokens(vec![
                        ExpressionToken::Literal(100.0)
                    ]),
                    Expression::from_tokens(vec![
                        ExpressionToken::Literal(1.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Literal(2.0),
                    ])
                ]
            };
        );
    }
}
