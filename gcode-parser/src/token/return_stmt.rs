use expression::{
    gcode::{expression, parameter},
    Expression, Parameter,
};
use nom::{
    bytes::streaming::tag_no_case,
    combinator::{map, opt},
    error::{context, ParseError},
    sequence::{preceded, separated_pair},
    IResult,
};

/// Which type of block this is
#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    ident: Parameter,
    value: Option<Expression<f32>>,
}

// named!(pub return_stmt<Span, Return>,
//     sep!(
//         space0,
//         do_parse!(
//             ident: preceded!(char_no_case!('O'), gcode_non_global_ident) >>
//             tag_no_case!("return") >>
//             value: opt!(gcode_expression) >>
//             ({
//                 Return {
//                     ident,
//                     value
//                 }
//             })
//         )
//     )
// );

pub fn return_stmt<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Return, E> {
    context(
        "return stmt",
        map(
            separated_pair(
                // TODO: Non-global-ident
                preceded(tag_no_case("O"), parameter),
                tag_no_case("return"),
                opt(expression),
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
                ident: Parameter::Numbered(100),
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
                ident: Parameter::Numbered(100),
                value: None
            };
        );
    }
}
