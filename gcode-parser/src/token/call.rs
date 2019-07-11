use crate::token::block::{parse_block_ident, BlockIdent};
use expression::{parser::gcode, Expression};
use nom::{
    bytes::complete::tag_no_case,
    character::complete::space0,
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
                parse_block_ident,
                preceded(space0, tag_no_case("call")),
                many0(preceded(space0, gcode::expression)),
            ),
            |(subroutine_ident, arguments)| Call {
                subroutine_ident,
                arguments,
            },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use expression::{ArithmeticOperator, Expression, ExpressionToken};

    #[test]
    fn parse_call_no_args() {
        let expd: Call<f32> = Call {
            subroutine_ident: 100.into(),
            arguments: Vec::new(),
        };

        assert_parse!(
            parser = call;
            input = "o100 call";
            expected = expd;
        );
    }

    // From `x-trim.ngc`
    #[test]
    fn realworld() {
        let expd: Call<f32> = Call {
            subroutine_ident: "touchoff".into(),
            arguments: vec![
                Expression::from_tokens(vec![ExpressionToken::Literal(0.1)]),
                Expression::from_tokens(vec![ExpressionToken::Literal(0.0)]),
                Expression::from_tokens(vec![ExpressionToken::Literal(0.08)]),
            ],
        };

        assert_parse!(
            parser = call;
            input = "o<touchoff> call [0.100] [0] [0.08] (Touchoff and start cutting)";
            expected = expd;
            // Comments are ignored
            remaining = " (Touchoff and start cutting)"
        );
    }

    #[test]
    fn parse_call() {
        assert_parse!(
            parser = call;
            input = "o100 call [100] [1 + 2]";
            expected = Call {
                subroutine_ident: 100.into(),
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

    #[test]
    fn parse_call_no_spaces() {
        assert_parse!(
            parser = call;
            input = "o<arc2>call[100][1 + 2]";
            expected = Call {
                subroutine_ident: "arc2".into(),
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
