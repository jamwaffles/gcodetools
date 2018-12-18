#[macro_use]
extern crate criterion;
extern crate gcode_parser;
extern crate nom;

use criterion::Criterion;
use gcode_parser::program;
use nom::types::CompleteByteSlice as Cbs;
use std::time::Duration;

fn bench_center_arc(c: &mut Criterion) {
    c.bench_function("Center format arc", |b| {
        b.iter(|| {
            program(Cbs(b"X5.0417 Y1.9427Z2.123 I-0.3979 J0.3028 P99")).unwrap();
        })
    });
}

fn bench_radius_arc(c: &mut Criterion) {
    c.bench_function("Radius format arc", |b| {
        b.iter(|| {
            program(Cbs(b"r1.997999 x1.613302 y-1.178668 z15.0000 P99")).unwrap();
        })
    });
}

fn bench_arc_parse(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Parse multiple arc samples",
        |b, input| {
            b.iter(|| {
                program(Cbs(input.as_bytes())).unwrap();
            })
        },
        vec![
            "X5.0417Y1.9427I-0.3979J0.3028",
            "X1 Y1 z 20 I20 J0",
            "Y1 Z2 X5 J3 K4 P6",
            "Y20.9595 Z-0.5838 I-1.5875 J0.0066",
            "i.5 j.5",
            "r1.997999 x1.613302 y-1.178668",
            "X10 Y15 R20 Z5",
        ],
    );
}

criterion_group!{
    name = arc;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(100))
        .sample_size(100)
        .measurement_time(Duration::from_millis(3000));
    targets = bench_center_arc, bench_radius_arc, bench_arc_parse
}
criterion_main!(arc);
