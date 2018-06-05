#[macro_use]
extern crate criterion;
extern crate gcode_parser;
extern crate nom;

use nom::types::CompleteByteSlice as Cbs;

use criterion::Criterion;
use gcode_parser::tokenizer::test_prelude::*;

fn bench_one_of_no_case(c: &mut Criterion) {
    c.bench_function("One of no case", |b| {
        b.iter(|| {
            one_of_no_case(Cbs(b"A"), "abcd").unwrap();
        })
    });
}

fn bench_g_code(c: &mut Criterion) {
    c.bench_function("G code", |b| {
        b.iter(|| {
            g(Cbs(b"G99"), 99.0).unwrap();
        })
    });
}

fn bench_m_code(c: &mut Criterion) {
    c.bench_function("M code", |b| {
        b.iter(|| {
            m(Cbs(b"M99"), 99.0).unwrap();
        })
    });
}

criterion_group!(helpers, bench_one_of_no_case, bench_g_code, bench_m_code);
criterion_main!(helpers);
