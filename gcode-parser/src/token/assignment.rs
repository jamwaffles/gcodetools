use crate::value::{value, Value};
use expression::{parser::gcode, Parameter};
use nom::{
    branch::{alt, permutation},
    bytes::streaming::{tag, tag_no_case, take_until},
    character::streaming::{char, digit1, multispace0, space0},
    combinator::{map, map_res, opt},
    error::{context, ParseError},
    multi::many1,
    number::streaming::float,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

/// Assign a value to a variable
///
/// A value can be a literal or a complete expression
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    /// The parameter to assign a value to
    lhs: Parameter,

    /// The value or result of an expression to assign
    rhs: Value,
}

// named!(pub assignment<Span, Assignment>,
//     map!(
//         sep!(
//             space0,
//             separated_pair!(
//                 gcode_parameter,
//                 char!('='),
//                 ngc_float_value
//             )
//         ),
//         |(lhs, rhs)| Assignment { lhs, rhs }
//     )
// );

pub fn assignment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Assignment, E> {
    context(
        "assignment",
        map(
            separated_pair(
                gcode::parameter,
                delimited(multispace0, char('='), multispace0),
                value,
            ),
            |(lhs, rhs)| Assignment { lhs, rhs },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use expression::{ArithmeticOperator, Expression, ExpressionToken};

    // TODO: Make this test work again
    // #[test]
    // fn parse_assignment() {
    //     assert_parse!(
    //         parser = assignment;
    //         input =
    //             "#1000 = 1.0",
    //             "#<named> = [1 + 2]"
    //         ;
    //         expected =
    //             Assignment {
    //                 lhs: Parameter::Numbered(1000),
    //                 rhs: 1.0.into()
    //             },
    //             Assignment {
    //                 lhs: Parameter::Local("named".into()),
    //                 rhs: Value::Expression(Expression::from_tokens(vec![
    //                     ExpressionToken::Literal(1.0),
    //                     ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
    //                     ExpressionToken::Literal(2.0),
    //                 ]))
    //             }
    //         ;
    //     );
    // }
}
