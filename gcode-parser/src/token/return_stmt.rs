use super::block::{block_ident, BlockIdent};
use expression::{gcode::expression, Expression};
use nom::{
    bytes::complete::tag_no_case,
    character::complete::space1,
    combinator::{map, opt},
    error::{context, ParseError},
    sequence::{preceded, separated_pair},
    IResult,
};

/// Which type of block this is
#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    ident: BlockIdent,
    value: Option<Expression<f32>>,
}

pub fn return_stmt<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Return, E> {
    context(
        "return stmt",
        map(
            separated_pair(
                block_ident,
                preceded(space1, tag_no_case("return")),
                opt(preceded(space1, expression)),
            ),
            |(ident, value)| Return { ident, value },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use expression::{ArithmeticOperator, Expression, ExpressionToken};

    #[test]
    fn parse_return() {
        assert_parse!(
            parser = return_stmt;
            input = "o100 return [1 + 2]";
            expected = Return {
                ident: 100.into(),
                value: Some(Expression::from_tokens(vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                    ExpressionToken::Literal(2.0),
                ]))
            };
        );
    }

    #[test]
    fn parse_return_no_value() {
        assert_parse!(
            parser = return_stmt;
            input = "o100 return";
            expected = Return {
                ident: 100.into(),
                value: None
            };
        );
    }
}
