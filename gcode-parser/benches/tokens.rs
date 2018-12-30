extern crate criterion;

use criterion::*;
use gcode_parser::{
    dev::{center_format_arc, coord},
    Span,
};
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

fn token_center_format_arc(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "center format arc",
        |b, input| {
            b.iter(|| {
                center_format_arc(Span::new(CompleteByteSlice(input.as_bytes()))).unwrap();
            })
        },
        vec![
            "X0 Y1 I2 J3",
            "X0 Y1 I2 J3 P5",
            "X0 Y0 z 20 I20 J0",
            "X-2.4438 Y-0.2048 I-0.0766 J0.2022",
        ],
    );
}

criterion_group!(tokens, token_coord, token_center_format_arc);
criterion_main!(tokens);
