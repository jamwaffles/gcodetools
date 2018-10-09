use super::value::{float_value, Value};
use expression::{parser::gcode_parameter, Parameter};
use nom::types::CompleteByteSlice;
use nom::*;

use super::Token;

named!(parameter_assignment<CompleteByteSlice, (Parameter, Value)>, ws!(
    do_parse!(
        param: gcode_parameter >>
        char!('=') >>
        value: float_value >>
        ((param, value))
    )
));

named!(pub parameters<CompleteByteSlice, Token>, alt_complete!(
    // Order matters
    map!(parameter_assignment, |res| Token::ParameterAssignment(res)) |
    map!(gcode_parameter, |res| Token::Parameter(res))
));

#[cfg(test)]
mod tests {
    use super::super::expression::{ArithmeticOperator, ExpressionToken};
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    #[test]
    fn it_parses_parameter_assignment() {
        assert_complete_parse!(
            parameter_assignment(Cbs(b"#1234 = 4.5")),
            (Parameter::Numbered(1234u32), Value::Float(4.5f32))
        );
        assert_complete_parse!(
            parameter_assignment(Cbs(b"#1234 = 4")),
            (Parameter::Numbered(1234u32), Value::Float(4.0f32))
        );
        assert_complete_parse!(
            parameter_assignment(Cbs(b"#1234=4.5")),
            (Parameter::Numbered(1234u32), Value::Float(4.5f32))
        );
        assert_complete_parse!(
            parameter_assignment(Cbs(b"#<foo_bar> = 4.5")),
            (Parameter::Named("foo_bar".into()), Value::Float(4.5f32))
        );
        assert_complete_parse!(
            parameter_assignment(Cbs(b"#<_bar_baz> = 4.5")),
            (Parameter::Global("bar_baz".into()), Value::Float(4.5f32))
        );
    }

    #[test]
    fn it_parses_parameter_to_parameter_assignments() {
        assert_complete_parse!(
            parameter_assignment(Cbs(b"#<toolno>     =  #1")),
            (
                Parameter::Named("toolno".into()),
                Value::Parameter(Parameter::Numbered(1))
            )
        );
    }

    #[test]
    fn it_parses_expression_assignments() {
        assert_complete_parse!(
            parameter_assignment(Cbs(b"#<_bar_baz> = [1 + 2]")),
            (
                Parameter::Global("bar_baz".into()),
                Value::Expression(vec![
                    ExpressionToken::Literal(1.0),
                    ExpressionToken::ArithmeticOperator(ArithmeticOperator::Add),
                    ExpressionToken::Literal(2.0),
                ])
            )
        );
    }

    #[test]
    fn it_parses_usages() {
        assert_eq!(
            parameters(Cbs(b"#1234")),
            Ok((EMPTY, Token::Parameter(Parameter::Numbered(1234u32))))
        );
        assert_eq!(
            parameters(Cbs(b"#<foo_bar>")),
            Ok((EMPTY, Token::Parameter(Parameter::Named("foo_bar".into()))))
        );
        assert_eq!(
            parameters(Cbs(b"#<_baz_quux>")),
            Ok((
                EMPTY,
                Token::Parameter(Parameter::Global("baz_quux".into()))
            ))
        );
    }
}
