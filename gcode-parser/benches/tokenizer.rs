#[macro_use]
extern crate criterion;
extern crate gcode_parser;
extern crate nom;

use nom::types::CompleteByteSlice as Cbs;

use criterion::Criterion;
use gcode_parser::tokenizer::test_prelude::*;
use gcode_parser::tokenizer::Tokenizer;

/// Benchmark parsing a contrived, but real-world-ish example
fn parse_linear_program(c: &mut Criterion) {
    c.bench_function("Parse simple linear program", |b| {
        let program = r#"
            G21 G54
            M3 S1000
            G0 X0 Y0 Z10
            F500
            G1 Z0
            G1 Y10
            X10 Y10
            X0 Y10
            X0 Y0
            G1 Z10
            G0 Z20
            M5
            M2
        "#;

        b.iter(|| {
            let tokenizer = Tokenizer::new_from_str(&program);

            tokenizer.tokenize().unwrap();
        })
    });
}

/// Parse a coordinate
fn parse_vec9(c: &mut Criterion) {
    c.bench_function("Parse 9-dimension vectors", |b| {
        // An example coord written by a drunk programmer to test all code paths
        let program = b"X0.1 y1.0z3.5 A 4 B5 c6 u7 V 8 W9";

        b.iter(|| {
            vec9(Cbs(program)).unwrap();
        })
    });
}

/// Parse a center format arc
fn parse_arc(c: &mut Criterion) {
    c.bench_function("Parse center format arcs", |b| {
        let program = b"X5.0417Y1.9427Z10.00000I-0.3979J0.3028l1.23456789";

        b.iter(|| {
            center_format_arc(Cbs(program)).unwrap();
        })
    });
}

criterion_group!(tokenizer, parse_linear_program, parse_vec9, parse_arc);
criterion_main!(tokenizer);
