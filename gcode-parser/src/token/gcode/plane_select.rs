use crate::{map_code, Span};
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
    alt_complete!(
        map_code!("G17", |_| PlaneSelect { plane: PlaneSelectValue::XY }) |
        map_code!("G18", |_| PlaneSelect { plane: PlaneSelectValue::ZX }) |
        map_code!("G19", |_| PlaneSelect { plane: PlaneSelectValue::YZ }) |
        map_code!("G17.1", |_| PlaneSelect { plane: PlaneSelectValue::UV }) |
        map_code!("G18.1", |_| PlaneSelect { plane: PlaneSelectValue::WU }) |
        map_code!("G19.1", |_| PlaneSelect { plane: PlaneSelectValue::VW })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer_plane_select() {
        let raw = span!(b"G17");

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
        let raw = span!(b"G17.1");

        assert_parse!(
            parser = plane_select;
            input = raw;
            expected = PlaneSelect {
                plane: PlaneSelectValue::UV
            }
        );
    }
}
