#[macro_use]
extern crate criterion;
extern crate gcode_parser;
extern crate nom;

use nom::types::CompleteByteSlice as Cbs;

use criterion::Criterion;
use gcode_parser::program;

fn parse_expression(c: &mut Criterion) {
    c.bench_function("Parse expression", |b| {
        let expr = r#"
        1 + #1234 / 3 * 4 - 5 +
        sin[
            5 + #<named> * [
                cos[4] + #<_global>
            ]
        ]
        ]"#;

        b.iter(|| {
            program(Cbs(expr.as_bytes())).unwrap();
        })
    });
}

criterion_group!(expression, parse_expression,);
criterion_main!(expression);
