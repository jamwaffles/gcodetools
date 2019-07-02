use expression::{parser::gcode, Expression, Parameter};
use nom::{
    bytes::streaming::tag_no_case,
    character::streaming::multispace0,
    combinator::map,
    error::{context, ParseError},
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::str::FromStr;

/// Which type of block this is
#[derive(Debug, PartialEq, Clone)]
pub struct Call<T> {
    subroutine_ident: Parameter,
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
                // TODO: Re-add `non_global_ident` function; this should only accept a non global
                // `<identifier>` without a hash. Probably want to call it `subroutine_ident` or
                // something.
                preceded(tag_no_case("O"), gcode::parameter),
                delimited(multispace0, tag_no_case("call"), multispace0),
                many0(gcode::expression),
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
    use expression::{ArithmeticOperator, Expression, ExpressionToken, Parameter};

    #[test]
    fn parse_call() {
        assert_parse!(
            parser = call;
            input = "o100 call [100] [1 + 2]";
            expected = Call {
                subroutine_ident: Parameter::Numbered(100),
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
