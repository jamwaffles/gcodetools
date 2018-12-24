//! Parse coordinates into a vector

use crate::Span;
use nom::types::CompleteByteSlice;
use nom::*;
use nom_locate::position;

#[derive(Debug, PartialEq, Clone)]
pub struct Coord<'a> {
    pub(crate) span: Span<'a>,
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

impl<'a> Default for Coord<'a> {
    fn default() -> Self {
        Self {
            span: Span::new(CompleteByteSlice(b"")),
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

// TODO: Test this in benchmarks
// static EMPTY_COORD: Coord = Coord {
//     x: None,
//     y: None,
//     z: None,
//     a: None,
//     b: None,
//     c: None,
//     u: None,
//     v: None,
//     w: None,
// };

named!(pub coord<Span, Coord>,
    map_res!(
        tuple!(
            position!(),
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
            )
        ),
        |(span, parts)| {
            let (x, y, z, a, b, c, u, v, w) = parts;

            let coord = Coord { span, x, y, z, a, b, c, u, v, w };

            // TODO: Benchmark this against static `EMPTY_COORD`
            let empty = Coord::default();

            if coord == empty {
                Err(())
            } else {
                Ok(coord)
            }
        })
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_xyz() {
        assert_parse!(
            parser = coord,
            input = span!(b"X0.0 Y1.0 Z2.0"),
            expected = Coord {
                span: empty_span!(),
                x: Some(0.0),
                y: Some(1.0),
                z: Some(2.0),
                ..Coord::default()
            },
            remaining = empty_span!(offset = 14)
        );
    }

    #[test]
    fn parse_lowercase() {
        assert_parse!(
            parser = coord,
            input = span!(b"x0.0 y1.0 z2.0"),
            expected = Coord {
                span: empty_span!(),
                x: Some(0.0),
                y: Some(1.0),
                z: Some(2.0),
                ..Coord::default()
            },
            remaining = empty_span!(offset = 14)
        );
    }

    #[test]
    fn parse_lowercase_integers() {
        assert_parse!(
            parser = coord,
            input = span!(b"x12 y34 z56"),
            expected = Coord {
                span: empty_span!(),
                x: Some(12.0),
                y: Some(34.0),
                z: Some(56.0),
                ..Coord::default()
            },
            remaining = empty_span!(offset = 11)
        );
    }

    #[test]
    fn parse_no_whitespace() {
        assert_parse!(
            parser = coord,
            input = span!(b"x12y34z56"),
            expected = Coord {
                span: empty_span!(),
                x: Some(12.0),
                y: Some(34.0),
                z: Some(56.0),
                ..Coord::default()
            },
            remaining = empty_span!(offset = 9)
        );
    }
}
