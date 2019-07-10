#[macro_use]
extern crate criterion;

use criterion::{Criterion, ParameterizedBenchmark};
use gcode_parser::dev::char_no_case;
use nom::bytes::complete::tag_no_case;

fn bench_fibs(c: &mut Criterion) {
    c.bench(
        "single char tag",
        ParameterizedBenchmark::new(
            "tag_no_case",
            |b, i| b.iter(|| tag_no_case::<_, _, ()>("A")(*i)),
            vec!["A", "a", "b", "abcd"],
        )
        .with_function("char_no_case", |b, i| {
            b.iter(|| char_no_case::<_, ()>('A')(*i))
        }),
    );
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
