#[macro_use]
extern crate criterion;
extern crate gcode_parser;
extern crate nom;

use criterion::Criterion;
use gcode_parser::tokenizer::test_prelude::*;
use nom::types::CompleteByteSlice as Cbs;
use std::time::Duration;

fn bench_one_of_no_case(c: &mut Criterion) {
    c.bench_function("One of no case", |b| {
        b.iter(|| {
            one_of_no_case(Cbs(b"A"), "abcd").unwrap();
        })
    });
}

fn bench_g_code(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "G codes",
        |b, input| {
            b.iter(|| {
                gcode(Cbs(input.as_bytes())).unwrap();
            })
        },
        vec!["G0", "G1", "G21", "G49"],
    );
}

fn bench_m_code(c: &mut Criterion) {
    c.bench_function("M code", |b| {
        b.iter(|| {
            m(Cbs(b"M99"), 99.0).unwrap();
        })
    });
}

criterion_group!{
    name = helpers;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(200))
        .sample_size(100)
        .measurement_time(Duration::from_millis(3000));
    targets = bench_one_of_no_case, bench_g_code, bench_m_code
}
criterion_main!(helpers);
