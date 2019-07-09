use crate::word::word;
use nom::{
    branch::alt,
    combinator::map,
    error::{context, ParseError},
    IResult,
};

// TODO: Better name than PlaneSelectValue
/// Which work offset to use
#[derive(Debug, PartialEq, Clone)]
pub enum PlaneSelectValue {
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

/// Plane select
#[derive(Debug, PartialEq, Clone)]
pub struct PlaneSelect {
    /// Which plane to work in
    pub plane: PlaneSelectValue,
}

pub fn plane_select<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PlaneSelect, E> {
    context(
        "plane select",
        map(
            alt((
                map(word("G17"), |_| PlaneSelectValue::XY),
                map(word("G18"), |_| PlaneSelectValue::ZX),
                map(word("G19"), |_| PlaneSelectValue::YZ),
                map(word("G17.1"), |_| PlaneSelectValue::UV),
                map(word("G18.1"), |_| PlaneSelectValue::WU),
                map(word("G19.1"), |_| PlaneSelectValue::VW),
            )),
            |plane| PlaneSelect { plane },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;

    #[test]
    fn parse_integer_plane_select() {
        let raw = "G17";

        assert_parse!(
            parser = plane_select;
            input = raw;
            expected = PlaneSelect {
                plane: PlaneSelectValue::XY
            }
        );
    }

    #[test]
    fn parse_decimal_plane_select() {
        let raw = "G17.1";

        assert_parse!(
            parser = plane_select;
            input = raw;
            expected = PlaneSelect {
                plane: PlaneSelectValue::UV
            }
        );
    }
}
