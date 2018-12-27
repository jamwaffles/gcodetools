extern crate criterion;

use criterion::*;
use gcode_parser::{dev::coord, Span};
use nom::types::CompleteByteSlice;

fn token_coord(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "coordinate",
        |b, input| {
            b.iter(|| {
                coord(Span::new(CompleteByteSlice(input.as_bytes()))).unwrap();
            })
        },
        vec![
            "X10 Y20 X30",
            "X0Y1Z2",
            "X-0.5 Y-2 Z100",
            "Z1",
            "X6.244 Y11.694 Z12.163",
            "x1.978000 x-0.118942 y-1.974421",
            "X6.244 Y11.694 Z12.163 a1.978000 b-0.118942 c-1.974421",
        ],
    );
}

criterion_group!(tokens, token_coord);
criterion_main!(tokens);
