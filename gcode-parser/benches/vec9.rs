#[macro_use]
extern crate criterion;
extern crate gcode_parser;
extern crate nom;

use criterion::Criterion;
use gcode_parser::program;
use nom::types::CompleteByteSlice as Cbs;
use std::time::Duration;

fn bench_vec9(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Parse multiple coordinate samples",
        |b, input| {
            b.iter(|| {
                program(Cbs(input.as_bytes())).unwrap();
            })
        },
        vec!["X10 Y20 X30", "X0Y1Z2", "X-0.5 Y-2 Z100", "Z1"],
    );
}

criterion_group! {
    name = vec9;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(100))
        .sample_size(100)
        .measurement_time(Duration::from_millis(3000));
    targets = bench_vec9
}
criterion_main!(vec9);
