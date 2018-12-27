extern crate criterion;

use criterion::*;
use gcode_parser::Program;

fn universal_gcode_sender_serial_stress_test(c: &mut Criterion) {
    let program = include_str!("../../test_files/universal_gcode_sender/serial_stress_test.gcode");

    c.bench(
        "universal_gcode_sender",
        Benchmark::new("serial_stress_test.gcode", move |b| {
            b.iter(|| Program::from_str(program))
        })
        .throughput(Throughput::Bytes(program.len() as u32)),
    );
}

fn universal_gcode_sender_buffer_stress_test(c: &mut Criterion) {
    let program = include_str!("../../test_files/universal_gcode_sender/buffer_stress_test.gcode");

    c.bench(
        "universal_gcode_sender",
        Benchmark::new("buffer_stress_test.gcode", move |b| {
            b.iter(|| Program::from_str(program))
        })
        .throughput(Throughput::Bytes(program.len() as u32)),
    );
}

criterion_group!(
    universal_gcode_sender,
    universal_gcode_sender_serial_stress_test,
    universal_gcode_sender_buffer_stress_test
);
criterion_main!(universal_gcode_sender);
