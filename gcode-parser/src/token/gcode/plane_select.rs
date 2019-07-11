use crate::word::word;
use nom::{
    branch::alt,
    combinator::map,
    error::{context, ParseError},
    IResult,
};

/// Which plane to use
#[derive(Debug, PartialEq, Clone)]
pub enum PlaneSelect {
    /// XY plane (`G17`)
    XY = 0,
    /// ZX plane (`G18`)
    ZX = 1,
    /// YZ plane (`G19`)
    YZ = 2,
    /// UV plane (`G17.1`)
    UV = 3,
    /// WU plane (`G18.1`)
    WU = 4,
    /// VW plane (`G19.1`)
    VW = 5,
}

pub fn plane_select<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PlaneSelect, E> {
    context(
        "plane select",
        alt((
            map(word("G17"), |_| PlaneSelect::XY),
            map(word("G18"), |_| PlaneSelect::ZX),
            map(word("G19"), |_| PlaneSelect::YZ),
            map(word("G17.1"), |_| PlaneSelect::UV),
            map(word("G18.1"), |_| PlaneSelect::WU),
            map(word("G19.1"), |_| PlaneSelect::VW),
        )),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;

    #[test]
    fn parse_integer_plane_select() {
        assert_parse!(
            parser = plane_select;
            input = "G17";
            expected = PlaneSelect::XY
        );
    }

    #[test]
    fn parse_decimal_plane_select() {
        assert_parse!(
            parser = plane_select;
            input = "G17.1";
            expected = PlaneSelect::UV
        );
    }
}
