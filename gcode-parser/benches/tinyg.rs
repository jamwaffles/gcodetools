extern crate criterion;

use criterion::*;
use gcode_parser::Program;

fn tinyg_mudflap_100in(c: &mut Criterion) {
    let program = include_str!("../../test_files/tinyg/mudflap_100in.gcode");

    c.bench(
        "tinyg",
        Benchmark::new("mudflap_100in.gcode", move |b| {
            b.iter(|| Program::from_str(program))
        })
        .throughput(Throughput::Bytes(program.len() as u32)),
    );
}

fn tinyg_zoetrope(c: &mut Criterion) {
    let program = include_str!("../../test_files/tinyg/zoetrope.gcode");

    c.bench(
        "tinyg",
        Benchmark::new("zoetrope.gcode", move |b| {
            b.iter(|| Program::from_str(program))
        })
        .throughput(Throughput::Bytes(program.len() as u32)),
    );
}

criterion_group!(tinyg, tinyg_mudflap_100in, tinyg_zoetrope);
criterion_main!(tinyg);
