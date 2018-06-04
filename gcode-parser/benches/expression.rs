#[macro_use]
extern crate criterion;
#[macro_use]
extern crate maplit;
extern crate gcode_parser;
extern crate nom;

use nom::types::CompleteByteSlice as Cbs;

use criterion::Criterion;
use gcode_parser::expression::evaluator::*;
use gcode_parser::tokenizer::test_prelude::*;

fn parse_and_evaluate(c: &mut Criterion) {
    c.bench_function("Parse and evaluate expression", |b| {
        let expr = r#"[
        1 + 2 / 3 * 4 - 5 +
        sin[
            5 + 6 * [
                cos[4] + 2.0
            ]
        ]
        ]"#;

        b.iter(|| {
            let parsed = parse_expression(Cbs(expr.as_bytes())).unwrap();

            evaluate(parsed.1, None)
        })
    });
}

fn parse_and_evaluate_with_context(c: &mut Criterion) {
    c.bench_function("Parse and evaluate expression with context", |b| {
        let context: Context = hashmap!{
            Parameter::Numbered(1234) => 1.2,
            Parameter::Named("named".into()) => 3.4,
            Parameter::Global("global".into()) => 5.6,
        };

        let expr = r#"[
        1 + #1234 / 3 * 4 - 5 +
        sin[
            5 + #<named> * [
                cos[4] + #<_global>
            ]
        ]
        ]"#;

        b.iter(|| {
            let parsed = parse_expression(Cbs(expr.as_bytes())).unwrap();

            evaluate(parsed.1, Some(&context))
        })
    });
}

criterion_group!(
    expression,
    parse_and_evaluate,
    parse_and_evaluate_with_context
);
criterion_main!(expression);