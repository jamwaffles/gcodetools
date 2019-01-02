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

named_attr!(
    #[doc = "Parse a numbered, local or global parameter"],
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
    use super::*;
    use nom::types::CompleteByteSlice as Cbs;

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
}
