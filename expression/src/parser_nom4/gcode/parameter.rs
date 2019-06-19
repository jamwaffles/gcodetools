use crate::Parameter;
use common::parsing::Span;
use nom::*;

named!(pub numbered_parameter_ident<Span, Parameter>, map!(
    flat_map!(digit, parse_to!(u32)),
    |res| Parameter::Numbered(res)
));

named!(pub named_parameter_ident<Span, Parameter>,
    map!(
        flat_map!(
            sep!(
                space0,
                delimited!(
                    char!('<'),
                    take_until!(">"),
                    char!('>')
                )
            ),
            parse_to!(String)
        ),
        |res| Parameter::Named(res)
    )
);

named!(global_parameter_ident<Span, Parameter>,
    map!(
        flat_map!(
            sep!(
                space0,
                delimited!(
                    tag!("<_"),
                    take_until!(">"),
                    char!('>')
                )
            ),
            parse_to!(String)
        ),
        |res| Parameter::Global(res)
    )
);

named_attr!(
    #[doc = "Parse a non-global identifier like `100` or `<foo>`

This is useful when parsing block markers like `O<foo>` or O100` where a global parameter cannot be
used."],
    pub non_global_ident<Span, Parameter>,
    alt_complete!(numbered_parameter_ident | named_parameter_ident)
);

named!(pub parameter_ident<Span, Parameter>,
    // Order is significant
    alt_complete!(numbered_parameter_ident | global_parameter_ident | named_parameter_ident)
);

named_attr!(
    #[doc = "Parse a numbered, local or global parameter"],
    pub parameter<Span, Parameter>,
    preceded!(char!('#'), parameter_ident)
);

named!(pub not_numbered_parameter_ident<Span, Parameter>,
    // Order is significant
    alt_complete!(global_parameter_ident | named_parameter_ident)
);

named!(
    pub not_numbered_parameter<Span, Parameter>,
    preceded!(char!('#'), not_numbered_parameter_ident)
);

#[cfg(test)]
mod tests {
    use super::*;
    use common::{assert_parse, span};

    #[test]
    fn it_parses_named_parameters() {
        assert_parse!(
            parser = parameter;
            input = span!(b"#<foo_bar>");
            expected = Parameter::Named("foo_bar".into());
        );
    }

    #[test]
    fn it_parses_not_numbered_parameters() {
        assert!(not_numbered_parameter(span!(b"#<foo_bar>")).is_ok());
        assert!(not_numbered_parameter(span!(b"#<_global>")).is_ok());
        assert!(not_numbered_parameter(span!(b"#1234")).is_err());
    }

    #[test]
    fn it_parses_global_parameters() {
        assert_parse!(
            parser = parameter;
            input = span!(b"#<_bar_baz>");
            expected = Parameter::Global("bar_baz".into());
        );
    }

    #[test]
    fn it_parses_parameters() {
        assert_parse!(
            parser = parameter;
            input =
                span!(b"#1234"),
                span!(b"#<foo_bar>"),
                span!(b"#<_bar_baz>")
            ;
            expected =
                Parameter::Numbered(1234u32),
                Parameter::Named("foo_bar".into()),
                Parameter::Global("bar_baz".into())
            ;
        );
    }

    #[test]
    fn it_parses_parameters_with_spaces_after_hash() {
        assert!(parameter(span!(b"# 1234")).is_err());

        assert_parse!(
            parser = parameter;
            input = span!(b"# <foo_bar>"), span!(b"# <_bar_baz>");
            expected = Parameter::Named("foo_bar".into()), Parameter::Global("bar_baz".into());
        );
    }
}
