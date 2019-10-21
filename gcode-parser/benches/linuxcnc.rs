extern crate criterion;

use criterion::*;
use gcode_parser::Program;

fn linuxcnc_skeleton_ngc(c: &mut Criterion) {
    let program = include_str!("../../test_files/linuxcnc/nc_files/skeleton.ngc");

    c.bench(
        "linuxcnc",
        Benchmark::new("skeleton.ngc", move |b| {
            b.iter(|| Program::from_str(program))
        })
        .throughput(Throughput::Bytes(program.len() as u64)),
    );
}

fn linuxcnc_b_index_ngc(c: &mut Criterion) {
    let program = include_str!("../../test_files/linuxcnc/nc_files/b-index.ngc");

    c.bench(
        "linuxcnc",
        Benchmark::new("b-index.ngc", move |b| {
            b.iter(|| Program::from_str(program))
        })
        .throughput(Throughput::Bytes(program.len() as u64)),
    );
}

fn linuxcnc_smartprobe(c: &mut Criterion) {
    let program = include_str!("../../test_files/linuxcnc/nc_files/smartprobe.ngc");

    c.bench(
        "linuxcnc",
        Benchmark::new("smartprobe.ngc", move |b| {
            b.iter(|| Program::from_str(program))
        })
        .throughput(Throughput::Bytes(program.len() as u64)),
    );
}

criterion_group!(
    linuxcnc,
    linuxcnc_skeleton_ngc,
    linuxcnc_b_index_ngc,
    linuxcnc_smartprobe
);
criterion_main!(linuxcnc);
