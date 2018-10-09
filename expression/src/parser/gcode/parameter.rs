use super::value::float_value;
use crate::value::Value;
use crate::Parameter;
use nom::types::CompleteByteSlice;
use nom::*;

named!(numbered_parameter<CompleteByteSlice, Parameter>, map!(
    flat_map!(preceded!(char!('#'), digit), parse_to!(u32)),
    |res| Parameter::Numbered(res)
));
named!(named_parameter<CompleteByteSlice, Parameter>, map!(
    flat_map!(delimited!(ws!(tuple!(char!('#'), char!('<'))), take_until!(">"), char!('>')), parse_to!(String)),
    |res| Parameter::Named(res)
));
named!(global_parameter<CompleteByteSlice, Parameter>, map!(
    flat_map!(delimited!(ws!(tuple!(char!('#'), tag!("<_"))), take_until!(">"), char!('>')), parse_to!(String)),
    |res| Parameter::Global(res)
));

named!(
    pub parameter<CompleteByteSlice, Parameter>,
    // Order is significant
    alt_complete!(numbered_parameter | global_parameter | named_parameter)
);

named!(
    pub not_numbered_parameter<CompleteByteSlice, Parameter>,
    alt_complete!(global_parameter | named_parameter)
);

#[cfg(test)]
mod tests {
    use super::super::expression::{ArithmeticOperator, ExpressionToken};
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    #[test]
    fn it_parses_named_parameters() {
        assert_complete_parse!(
            named_parameter(Cbs(b"#<foo_bar>")),
            Parameter::Named("foo_bar".into())
        );
    }

    #[test]
    fn it_parses_not_numbered_parameters() {
        assert!(not_numbered_parameter(Cbs(b"#<foo_bar>")).is_ok());
        assert!(not_numbered_parameter(Cbs(b"#<_global>")).is_ok());
        assert!(not_numbered_parameter(Cbs(b"#1234")).is_err());
    }

    #[test]
    fn it_parses_global_parameters() {
        assert_complete_parse!(
            global_parameter(Cbs(b"#<_bar_baz>")),
            Parameter::Global("bar_baz".into())
        );
    }

    #[test]
    fn it_parses_parameters() {
        assert_complete_parse!(parameter(Cbs(b"#1234")), Parameter::Numbered(1234u32));
        assert_complete_parse!(
            parameter(Cbs(b"#<foo_bar>")),
            Parameter::Named("foo_bar".into())
        );
        assert_complete_parse!(
            parameter(Cbs(b"#<_bar_baz>")),
            Parameter::Global("bar_baz".into())
        );
    }

    #[test]
    fn it_parses_parameters_with_spaces_after_hash() {
        assert!(parameter(Cbs(b"# 1234")).is_err());

        assert_complete_parse!(
            parameter(Cbs(b"# <foo_bar>")),
            Parameter::Named("foo_bar".into())
        );
        assert_complete_parse!(
            parameter(Cbs(b"# <_bar_baz>")),
            Parameter::Global("bar_baz".into())
        );
    }

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
