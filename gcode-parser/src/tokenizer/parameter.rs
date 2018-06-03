use nom::types::CompleteByteSlice;
use nom::*;

use super::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Parameter {
    Numbered(u32),
    Named(String),
    Global(String),
}

pub type ParameterValue = f32;

named!(numbered_parameter<CompleteByteSlice, Parameter>, map!(
    flat_map!(preceded!(char!('#'), digit), parse_to!(u32)),
    |res| Parameter::Numbered(res)
));
named!(named_parameter<CompleteByteSlice, Parameter>, map!(
    flat_map!(delimited!(tag!("#<"), take_until!(">"), char!('>')), parse_to!(String)),
    |res| Parameter::Named(res)
));
named!(global_parameter<CompleteByteSlice, Parameter>, map!(
    flat_map!(delimited!(tag!("#<_"), take_until!(">"), char!('>')), parse_to!(String)),
    |res| Parameter::Global(res)
));

named!(
    pub parameter<CompleteByteSlice, Parameter>,
    // Order is significant
    alt_complete!(numbered_parameter | global_parameter | named_parameter)
);

named!(parameter_assignment<CompleteByteSlice, (Parameter, ParameterValue)>, ws!(
    do_parse!(
        parameter: parameter >>
        char!('=') >>
        value: flat_map!(recognize_float, parse_to!(f32)) >>
        ((parameter, value))
    )
));

named!(pub parameters<CompleteByteSlice, Token>, alt_complete!(
    // Order matters
    map!(parameter_assignment, |res| Token::ParameterAssignment(res)) |
    map!(parameter, |res| Token::Parameter(res))
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

    const EMPTY: Cbs = Cbs(b"");

    #[test]
    fn it_parses_named_parameters() {
        assert_eq!(
            named_parameter(Cbs(b"#<foo_bar>")),
            Ok((EMPTY, Parameter::Named("foo_bar".into())))
        );
    }

    #[test]
    fn it_parses_global_parameters() {
        assert_eq!(
            global_parameter(Cbs(b"#<_bar_baz>")),
            Ok((EMPTY, Parameter::Global("bar_baz".into())))
        );
    }

    #[test]
    fn it_parses_parameters() {
        assert_eq!(
            parameter(Cbs(b"#1234")),
            Ok((EMPTY, Parameter::Numbered(1234u32)))
        );
        assert_eq!(
            parameter(Cbs(b"#<foo_bar>")),
            Ok((EMPTY, Parameter::Named("foo_bar".into())))
        );
        assert_eq!(
            parameter(Cbs(b"#<_bar_baz>")),
            Ok((EMPTY, Parameter::Global("bar_baz".into())))
        );
    }

    #[test]
    fn it_parses_parameter_assignment() {
        assert_eq!(
            parameter_assignment(Cbs(b"#1234 = 4.5")),
            Ok((EMPTY, (Parameter::Numbered(1234u32), 4.5f32)))
        );
        assert_eq!(
            parameter_assignment(Cbs(b"#1234 = 4")),
            Ok((EMPTY, (Parameter::Numbered(1234u32), 4.0f32)))
        );
        assert_eq!(
            parameter_assignment(Cbs(b"#1234=4.5")),
            Ok((EMPTY, (Parameter::Numbered(1234u32), 4.5f32)))
        );
        assert_eq!(
            parameter_assignment(Cbs(b"#<foo_bar> = 4.5")),
            Ok((EMPTY, (Parameter::Named("foo_bar".into()), 4.5f32)))
        );
        assert_eq!(
            parameter_assignment(Cbs(b"#<_bar_baz> = 4.5")),
            Ok((EMPTY, (Parameter::Global("bar_baz".into()), 4.5f32)))
        );
    }

    #[test]
    fn it_parses_assignments_and_usages() {
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

        assert_eq!(
            parameters(Cbs(b"#1234 = 4.5")),
            Ok((
                EMPTY,
                Token::ParameterAssignment((Parameter::Numbered(1234u32), 4.5f32))
            ))
        );
        assert_eq!(
            parameters(Cbs(b"#1234 = 4")),
            Ok((
                EMPTY,
                Token::ParameterAssignment((Parameter::Numbered(1234u32), 4.0f32))
            ))
        );
        assert_eq!(
            parameters(Cbs(b"#1234=4.5")),
            Ok((
                EMPTY,
                Token::ParameterAssignment((Parameter::Numbered(1234u32), 4.5f32))
            ))
        );
        assert_eq!(
            parameters(Cbs(b"#<foo_bar> = 4.5")),
            Ok((
                EMPTY,
                Token::ParameterAssignment((Parameter::Named("foo_bar".into()), 4.5f32))
            ))
        );
        assert_eq!(
            parameters(Cbs(b"#<_bar_baz> = 4.5")),
            Ok((
                EMPTY,
                Token::ParameterAssignment((Parameter::Global("bar_baz".into()), 4.5f32))
            ))
        );
    }
}
