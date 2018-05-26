#[macro_use]
extern crate criterion;
extern crate gcode_parser;
extern crate nom;

use nom::types::CompleteByteSlice as Cbs;

use criterion::Criterion;
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

            tokenizer.tokenize();
        })
    });
}

criterion_group!(tokenizer, parse_linear_program);
criterion_main!(tokenizer);
