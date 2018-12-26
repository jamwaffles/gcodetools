use crate::Span;
use nom::*;

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

named!(pub plane_select<Span, PlaneSelect>,
    map!(
        alt_complete!(
            map!(tag_no_case!("G17.1"), |_| PlaneSelectValue::UV) |
            map!(tag_no_case!("G18.1"), |_| PlaneSelectValue::WU) |
            map!(tag_no_case!("G19.1"), |_| PlaneSelectValue::VW) |
            map!(tag_no_case!("G17"), |_| PlaneSelectValue::XY) |
            map!(tag_no_case!("G18"), |_| PlaneSelectValue::ZX) |
            map!(tag_no_case!("G19"), |_| PlaneSelectValue::YZ)

        ),
        |plane| PlaneSelect { plane }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer_plane_select() {
        let raw = span!(b"G17");

        assert_parse!(
            parser = plane_select,
            input = raw,
            expected = PlaneSelect {
                plane: PlaneSelectValue::XY
            },
            remaining = empty_span!(offset = 3)
        );
    }

    #[test]
    fn parse_decimal_plane_select() {
        let raw = span!(b"G17.1");

        assert_parse!(
            parser = plane_select,
            input = raw,
            expected = PlaneSelect {
                plane: PlaneSelectValue::UV
            },
            remaining = empty_span!(offset = 5)
        );
    }
}
