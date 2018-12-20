//! Parse coordinates into a vector

use crate::Span;
use nom::types::CompleteByteSlice;
use nom::*;

#[derive(Debug, PartialEq)]
pub struct Coord {
    pub(crate) x: Option<f32>,
    pub(crate) y: Option<f32>,
    pub(crate) z: Option<f32>,
    pub(crate) a: Option<f32>,
    pub(crate) b: Option<f32>,
    pub(crate) c: Option<f32>,
    pub(crate) u: Option<f32>,
    pub(crate) v: Option<f32>,
    pub(crate) w: Option<f32>,
}

impl Default for Coord {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            a: None,
            b: None,
            c: None,
            u: None,
            v: None,
            w: None,
        }
    }
}

named!(pub coord<Span, Coord>,
    map!(
        sep!(
            space0,
            permutation!(
                opt!(preceded!(one_of!("Xx"), float)),
                opt!(preceded!(one_of!("Yy"), float)),
                opt!(preceded!(one_of!("Zz"), float)),
                opt!(preceded!(one_of!("Aa"), float)),
                opt!(preceded!(one_of!("Bb"), float)),
                opt!(preceded!(one_of!("Cc"), float)),
                opt!(preceded!(one_of!("Uu"), float)),
                opt!(preceded!(one_of!("Vv"), float)),
                opt!(preceded!(one_of!("Ww"), float))
            )
        ),
        |(x, y, z, a, b, c, u, v, w)| {
            Coord { x, y, z, a, b, c, u, v, w }
        }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_xyz() {
        assert_parse!(
            coord,
            Span::new(CompleteByteSlice(b"X0.0 Y1.0 Z2.0")),
            Coord {
                x: Some(0.0),
                y: Some(1.0),
                z: Some(2.0),
                a: None,
                b: None,
                c: None,
                u: None,
                v: None,
                w: None,
            },
            // Remaining
            Span {
                offset: 14,
                line: 1,
                fragment: CompleteByteSlice(b"")
            }
        );
    }

    #[test]
    fn parse_lowercase() {
        assert_parse!(
            coord,
            Span::new(CompleteByteSlice(b"x0.0 y1.0 z2.0")),
            Coord {
                x: Some(0.0),
                y: Some(1.0),
                z: Some(2.0),
                a: None,
                b: None,
                c: None,
                u: None,
                v: None,
                w: None,
            },
            // Remaining
            Span {
                offset: 14,
                line: 1,
                fragment: CompleteByteSlice(b"")
            }
        );
    }
}
