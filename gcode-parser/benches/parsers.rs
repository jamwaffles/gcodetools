#[macro_use]
extern crate criterion;

use criterion::{Criterion, ParameterizedBenchmark};
use gcode_parser::dev::{char_no_case, word};
use nom::bytes::complete::tag_no_case;

fn bench_char_no_case(c: &mut Criterion) {
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

fn bench_word(c: &mut Criterion) {
    c.bench(
        "word",
        ParameterizedBenchmark::new(
            "word",
            |b, i| b.iter(|| word::<()>("g43")(*i)),
            vec!["G43", "g61.5", "m30"],
        ),
    );
}

criterion_group!(benches, bench_char_no_case, bench_word);
criterion_main!(benches);
