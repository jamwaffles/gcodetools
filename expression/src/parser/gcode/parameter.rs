use crate::Parameter;
use common::parsing::Span;
use nom::*;

named!(numbered_parameter<Span, Parameter>, map!(
    flat_map!(preceded!(char!('#'), digit), parse_to!(u32)),
    |res| Parameter::Numbered(res)
));

named!(named_parameter<Span, Parameter>,
    map!(
        flat_map!(
            sep!(
                space0,
                preceded!(
                    char!('#'),
                    delimited!(
                        char!('<'),
                        take_until!(">"),
                        char!('>')
                    )
                )
            )
            ,
            parse_to!(String)
        ),
        |res| Parameter::Named(res)
    )
);

named!(global_parameter<Span, Parameter>,
    map!(
        flat_map!(
            sep!(
                space0,
                preceded!(
                    char!('#'),
                    delimited!(
                        tag!("<_"),
                        take_until!(">"),
                        char!('>')
                    )
                )
            )
            ,
            parse_to!(String)
        ),
        |res| Parameter::Global(res)
    )
);

named_attr!(
    #[doc = "Parse a numbered, local or global parameter"],
    pub parameter<Span, Parameter>,
    // Order is significant
    alt_complete!(numbered_parameter | global_parameter | named_parameter)
);

named!(
    pub not_numbered_parameter<Span, Parameter>,
    alt_complete!(global_parameter | named_parameter)
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
