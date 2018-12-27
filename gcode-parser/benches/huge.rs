extern crate criterion;

use criterion::*;
use gcode_parser::Program;
use std::time::Duration;

fn huge_tiger(c: &mut Criterion) {
    let program = include_str!("../../test_files/tinyg/tiger.gcode");

    c.bench(
        "huge files",
        Benchmark::new("tiger.gcode", move |b| {
            b.iter(|| Program::from_str(program))
        })
        .throughput(Throughput::Bytes(program.len() as u32)),
    );
}

criterion_group! {
    name = huge;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(1000))
        .sample_size(10)
        .measurement_time(Duration::from_millis(5000));
    targets = huge_tiger
}
criterion_main!(huge);
