use crate::{
    ArithmeticOperator, BinaryOperator, Expression, ExpressionToken, Function, LogicalOperator,
    Parameter,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until},
    character::complete::{char, digit1, multispace0, space0},
    combinator::{map, map_res},
    error::{context, ParseError},
    multi::many1,
    number::complete::recognize_float,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::str::FromStr;

/// Expression entry point
pub fn expression<'a, E: ParseError<&'a str>, V: FromStr>(
    i: &'a str,
) -> IResult<&'a str, Expression<V>, E> {
    context(
        "expression",
        map(
            delimited(char('['), many1(expression_token), char(']')),
            Expression::from_tokens,
        ),
    )(i)
}

fn expression_token<'a, E: ParseError<&'a str>, V: FromStr>(
    i: &'a str,
) -> IResult<&'a str, ExpressionToken<V>, E> {
    delimited(
        multispace0,
        alt((
            map(literal, ExpressionToken::Literal),
            map(operator, ExpressionToken::ArithmeticOperator),
            map(logical_operator, ExpressionToken::LogicalOperator),
            map(binary_operator, ExpressionToken::BinaryOperator),
            map(function, ExpressionToken::Function),
            map(exists, ExpressionToken::Function),
            map(expression, ExpressionToken::Expression),
            map(parameter, ExpressionToken::Parameter),
        )),
        multispace0,
    )(i)
}

fn literal<'a, E: ParseError<&'a str>, V: FromStr>(i: &'a str) -> IResult<&'a str, V, E> {
    map_res(recognize_float, |s| String::from(s).parse::<V>())(i)
}

fn operator<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ArithmeticOperator, E> {
    context(
        "operator",
        alt((
            map(char('/'), |_| ArithmeticOperator::Div),
            map(char('*'), |_| ArithmeticOperator::Mul),
            map(char('+'), |_| ArithmeticOperator::Add),
            map(char('-'), |_| ArithmeticOperator::Sub),
            map(tag_no_case("mod"), |_| ArithmeticOperator::Mod),
        )),
    )(i)
}

fn logical_operator<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, LogicalOperator, E> {
    context(
        "logical operator",
        alt((
            map(tag("AND"), |_| LogicalOperator::And),
            map(tag("OR"), |_| LogicalOperator::Or),
            map(tag("NOT"), |_| LogicalOperator::Not),
        )),
    )(i)
}

fn binary_operator<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, BinaryOperator, E> {
    context(
        "comparison",
        alt((
            map(tag_no_case("EQ"), |_| BinaryOperator::Equal),
            map(tag_no_case("NE"), |_| BinaryOperator::NotEqual),
            map(tag_no_case("GT"), |_| BinaryOperator::GreaterThan),
            map(tag_no_case("GE"), |_| BinaryOperator::GreaterThanOrEqual),
            map(tag_no_case("LT"), |_| BinaryOperator::LessThan),
            map(tag_no_case("LE"), |_| BinaryOperator::LessThanOrEqual),
        )),
    )(i)
}

pub fn parameter<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Parameter, E> {
    context(
        "parameter",
        preceded(
            char('#'),
            alt((
                map(
                    preceded(space0, delimited(tag("<_"), take_until(">"), char('>'))),
                    |s| Parameter::Global(String::from(s)),
                ),
                map(
                    preceded(space0, delimited(char('<'), take_until(">"), char('>'))),
                    |s| Parameter::Local(String::from(s)),
                ),
                map_res(digit1, |s| {
                    String::from(s).parse::<u32>().map(Parameter::Numbered)
                }),
            )),
        ),
    )(i)
}

/// `exists` is a special function that can only take a single parameter as an argument
fn exists<'a, E: ParseError<&'a str>, V>(i: &'a str) -> IResult<&'a str, Function<V>, E> {
    context(
        "exists",
        map(
            preceded(
                tag_no_case("exists"),
                delimited(
                    char('['),
                    delimited(multispace0, parameter, multispace0),
                    char(']'),
                ),
            ),
            Function::Exists,
        ),
    )(i)
}

fn function<'a, E: ParseError<&'a str>, V: FromStr>(
    i: &'a str,
) -> IResult<&'a str, Function<V>, E> {
    context(
        "function",
        alt((
            map(preceded(tag_no_case("abs"), expression), Function::Abs),
            map(preceded(tag_no_case("acos"), expression), Function::Acos),
            map(preceded(tag_no_case("asin"), expression), Function::Asin),
            map(
                separated_pair(
                    tag_no_case("atan"),
                    space0,
                    separated_pair(expression, char('/'), expression),
                ),
                |(_, args)| Function::Atan(args),
            ),
            map(preceded(tag_no_case("cos"), expression), Function::Cos),
            map(preceded(tag_no_case("exp"), expression), Function::Exp),
            map(
                // Aka "floor"
                preceded(tag_no_case("fix"), expression),
                Function::Floor,
            ),
            map(
                // Aka "ceil"
                preceded(tag_no_case("fup"), expression),
                Function::Ceil,
            ),
            map(preceded(tag_no_case("ln"), expression), Function::Ln),
            map(preceded(tag_no_case("round"), expression), Function::Round),
            map(preceded(tag_no_case("sin"), expression), Function::Sin),
            map(preceded(tag_no_case("sqrt"), expression), Function::Sqrt),
            map(preceded(tag_no_case("tan"), expression), Function::Tan),
        )),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_parse, assert_parse_ok};
    use nom::{
        error::{convert_error, VerboseError},
        Err,
    };

    #[test]
    fn parse_numbered_parameter() -> Result<(), String> {
        let numbered = "#123\n";

        let (remaining, result) =
            parameter::<VerboseError<&str>>(numbered).map_err(|e| match e {
                Err::Error(e) | Err::Failure(e) => convert_error(numbered, e),
                e => format!("Failed to parse: {:?}", e),
            })?;

        assert_eq!(remaining, "\n");
        assert_eq!(result, Parameter::Numbered(123u32));

        Ok(())
    }

    #[test]
    fn parse_local_parameter() -> Result<(), String> {
        let local = "#<local>";

        let (remaining, result) = parameter::<VerboseError<&str>>(local).map_err(|e| match e {
            Err::Error(e) | Err::Failure(e) => convert_error(local, e),
            e => format!("Failed to parse: {:?}", e),
        })?;

        assert_eq!(remaining.len(), 0);
        assert_eq!(result, Parameter::Local("local".into()));

        Ok(())
    }

    #[test]
    fn parse_global_parameter() -> Result<(), String> {
        let global = "#<_global>";

        let (remaining, result) = parameter::<VerboseError<&str>>(global).map_err(|e| match e {
            Err::Error(e) | Err::Failure(e) => convert_error(global, e),
            e => format!("Failed to parse: {:?}", e),
        })?;

        assert_eq!(remaining.len(), 0);
        assert_eq!(result, Parameter::Global("global".into()));

        Ok(())
    }

    #[test]
    fn it_parses_named_parameters() {
        assert_parse!(
            parser = parameter;
            input = "#<foo_bar>";
            expected = Parameter::Local("foo_bar".into());
        );
    }

    #[test]
    fn it_parses_global_parameters() {
        assert_parse!(
            parser = parameter;
            input = "#<_bar_baz>";
            expected = Parameter::Global("bar_baz".into());
        );
    }

    #[test]
    fn it_parses_parameters() {
        assert_parse!(
            parser = parameter;
            input =
                "#1234\n",
                "#<foo_bar>",
                "#<_bar_baz>"
            ;
            expected =
                Parameter::Numbered(1234u32),
                Parameter::Local("foo_bar".into()),
                Parameter::Global("bar_baz".into())
            ;
            remaining =
                "\n",
                "",
                ""
            ;
        );
    }

    #[test]
    fn it_parses_parameters_with_spaces_after_hash() {
        assert!(parameter::<VerboseError<&str>>("# 1234").is_err());

        assert_parse!(
            parser = parameter;
            input = "# <foo_bar>", "# <_bar_baz>";
            expected = Parameter::Local("foo_bar".into()), Parameter::Global("bar_baz".into());
        );
    }

    #[test]
    fn arithmetic_operators_have_the_right_precedence() {
        assert!(ArithmeticOperator::Div > ArithmeticOperator::Mul);
        assert!(ArithmeticOperator::Mul > ArithmeticOperator::Add);
        assert!(ArithmeticOperator::Add > ArithmeticOperator::Sub);
        assert!(ArithmeticOperator::Add == ArithmeticOperator::Add);
    }

    #[test]
    fn it_parses_simple_expressions() {
        assert_parse!(
            parser = expression;
            input = "[1]";
            expected = vec![ExpressionToken::Literal(1.0)].into()
        );
    }

    #[test]
    fn modulo() {
        assert_parse!(
            parser = expression;
            input = "[10 mod 3]";
            expected = vec![
                ExpressionToken::Literal(10.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mod),
                ExpressionToken::Literal(3.0),
            ].into()
        );
    }

    #[test]
    fn it_parses_arithmetic() {
        assert_parse!(
            parser = expression;
            input = "[1 + 2 * 3 / 4 - 5]";
            expected = vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Literal(2.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(3.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Div),
                ExpressionToken::Literal(4.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
                ExpressionToken::Literal(5.0),
            ].into()
        );
    }

    #[test]
    fn whitespace() {
        assert_parse!(
            parser = expression;
            input = "[ 1 + 2 * 3 / 4 - 5 ]";
            expected = vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Literal(2.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(3.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Div),
                ExpressionToken::Literal(4.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
                ExpressionToken::Literal(5.0),
            ].into()
        );
    }

    #[test]
    fn it_parses_nested_expressions() {
        assert_parse!(
            parser = expression;
            input = "[1 + [[2 - 3] * 4]]";
            expected = vec![
                ExpressionToken::Literal(1.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                ExpressionToken::Expression(vec![
                    ExpressionToken::Expression(vec![
                        ExpressionToken::Literal(2.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Sub),
                        ExpressionToken::Literal(3.0),
                    ].into()),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                    ExpressionToken::Literal(4.0),
                ].into()),
            ].into()
        );
    }

    #[test]
    fn it_parses_atan() {
        assert_parse!(
            parser = expression;
            input = "[ATAN[3 + 4]/[5]]";
            expected =
                vec![ExpressionToken::Function(Function::Atan((
                    vec![
                        ExpressionToken::Literal(3.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Literal(4.0),
                    ].into(),
                    vec![ExpressionToken::Literal(5.0)].into(),
                )))].into();
        );
    }

    #[test]
    fn it_parses_atan_with_spaces() {
        assert_parse!(
            parser = expression;
            input = "[atan [3 + 4]/[5]]";
            expected =
                vec![ExpressionToken::Function(Function::Atan((
                    vec![
                        ExpressionToken::Literal(3.0),
                        ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                        ExpressionToken::Literal(4.0),
                    ].into(),
                    vec![ExpressionToken::Literal(5.0)].into(),
                )))].into();
        );
    }

    #[test]
    fn it_parses_a_function() {
        assert_parse!(
            parser = expression;
            input =
                "[ABS[1.0]]"
            ;
            expected =
                vec![ExpressionToken::Function(Function::Abs(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into()
            ;
        );
    }

    #[test]
    fn it_parses_functions() {
        assert_parse!(
            parser = expression;
            input =
                "[ABS[1.0]]",
                "[ACOS[1.0]]",
                "[ASIN[1.0]]",
                "[COS[1.0]]",
                "[EXP[1.0]]",
                "[FIX[1.0]]",
                "[FUP[1.0]]",
                "[ROUND[1.0]]",
                "[LN[1.0]]",
                "[SIN[1.0]]",
                "[SQRT[1.0]]",
                "[TAN[1.0]]",
                "[EXISTS[#<named>]]"
            ;

            expected =
                vec![ExpressionToken::Function(Function::Abs(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Acos(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Asin(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Cos(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Exp(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Floor(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Ceil(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Round(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Ln(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Sin(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Sqrt(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Tan(vec![
                    ExpressionToken::Literal(1.0),
                ].into()))].into(),
                vec![ExpressionToken::Function(Function::Exists(Parameter::Local("named".into())))].into()
            ;
        );
    }

    #[test]
    fn it_parses_binary_operators() {
        assert_parse!(
            parser = expression;
            input =
                "[1 EQ 2]",
                "[1 NE 2]",
                "[1 GT 2]",
                "[1 GE 2]",
                "[1 LT 2]",
                "[1 LE 2]"
            ;
            expected =
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::Equal), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::NotEqual), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::GreaterThanOrEqual), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::LessThan), ExpressionToken::Literal(2.0)].into(),
                vec![ExpressionToken::Literal(1.0), ExpressionToken::BinaryOperator(BinaryOperator::LessThanOrEqual), ExpressionToken::Literal(2.0)].into()
            ;
        );
    }

    #[test]
    fn it_parses_logical_operators() {
        assert_parse!(
            parser = expression;
            input =
                "[1 AND 2]",
                "[1 OR 2]",
                "[1 NOT 2]",
                "[[#<fraction> GT .99] OR [#<fraction> LT .01]]"
            ;
            expected =
                vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::LogicalOperator(LogicalOperator::And),
                    ExpressionToken::Literal(2.0),
                ].into(),
                vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::LogicalOperator(LogicalOperator::Or),
                    ExpressionToken::Literal(2.0),
                ].into(),
                vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::LogicalOperator(LogicalOperator::Not),
                    ExpressionToken::Literal(2.0),
                ].into(),
                vec![
                    ExpressionToken::Expression(vec![
                        ExpressionToken::Parameter(Parameter::Local("fraction".into())),
                        ExpressionToken::BinaryOperator(BinaryOperator::GreaterThan),
                        ExpressionToken::Literal(0.99),
                    ].into()),
                    ExpressionToken::LogicalOperator(LogicalOperator::Or),
                    ExpressionToken::Expression(vec![
                        ExpressionToken::Parameter(Parameter::Local("fraction".into())),
                        ExpressionToken::BinaryOperator(BinaryOperator::LessThan),
                        ExpressionToken::Literal(0.01),
                    ].into()),
                ].into()
            ;
        );
    }

    #[test]
    fn it_parses_negative_numbers_as_negative_numbers() {
        assert_parse!(
            parser = expression;
            input = "[-10.0*-12]";
            expected = vec![
                ExpressionToken::Literal(-10.0),
                ExpressionToken::ArithmeticOperator(ArithmeticOperator::Mul),
                ExpressionToken::Literal(-12.0),
            ].into();
        );
    }

    #[test]
    fn it_parses_expressions_with_parameters() {
        assert_parse_ok!(
            parser = expression::<VerboseError<&str>, f64>,
            input = "[1 + #1234 * #<named_param> / #<_global_param>]"
        );
    }

    #[test]
    fn it_parses_function_calls() {
        assert_parse_ok!(
            parser = expression::<VerboseError<&str>, f64>,
            input = "[SIN[10]]"
        );
    }

    #[test]
    fn it_parses_exists_calls() {
        assert_parse!(
            parser = expression;
            input = "[EXISTS[#<named_param>]]";
            expected = vec![ExpressionToken::Function::<f64>(Function::Exists(
                Parameter::Local("named_param".into()),
            ))].into();
        );
    }
}
