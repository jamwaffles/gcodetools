//! TODO: Move to gcode _interpreter_ crate

#[macro_use]
extern crate criterion;
#[macro_use]
extern crate maplit;
extern crate nom;

use criterion::Criterion;
use expression::{evaluate, gcode_expression, Context, Parameter};
use nom::error::convert_error;
use nom::error::VerboseError;
use nom::Err;

fn parse_and_evaluate(c: &mut Criterion) {
    c.bench_function("Parse and evaluate expression", |b| {
        let expr = r#"[ 1 + 2 / 3 * 4 - 5 + sin[5 + 6 * [cos[4] + 2.0 ] ] ]"#;
        b.iter(|| {
            let parsed = gcode_expression::<VerboseError<&str>, f32>(expr).unwrap();

            evaluate(parsed.1, None)
        })
    });
}

fn parse_and_evaluate_with_context(c: &mut Criterion) {
    c.bench_function("Parse and evaluate expression with context", |b| {
        let context: Context<f32> = hashmap! {
            Parameter::Numbered(1234) => 1.2,
            Parameter::Local("named".into()) => 3.4,
            Parameter::Global("global".into()) => 5.6,
        };

        let expr = r#"[ 1 + #1234 / 3 * 4 - 5 + sin[5 + #<named> * [cos[4] + #<_global> ] ] ]"#;

        b.iter(|| {
            let parsed = gcode_expression::<VerboseError<&str>, f32>(expr)
                .map_err(|e| match e {
                    Err::Error(e) | Err::Failure(e) => {
                        let e = convert_error(expr, e);
                        println!("{}", e);
                        e
                    }
                    _ => String::from("Failed to parse for unknown reason"),
                })
                .unwrap();

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
