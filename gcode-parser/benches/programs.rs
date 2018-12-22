#[macro_use]
extern crate criterion;

use criterion::Criterion;
use gcode_parser::from_str;

fn bench_10_xyz_rapids(c: &mut Criterion) {
    let program = r#"G0 x0 y0 z0
x1.0 y2.0 z2.0
x1.1 y2.1 z2.1
x3.1 y3.1 z3.1
X8.7654321 Y12.3456789 Z3.456789
X9.7654321 Y15.3456789 Z9.456789
x1.0 y2.0 z2.0
x1.1 y2.1 z2.1
x3.1 y3.1 z3.1
X8.7654321 Y12.3456789 Z3.456789
X9.7654321 Y15.3456789 Z9.456789
m2"#;

    c.bench_function("10 XYZ rapids", move |b| b.iter(|| from_str(program)));
}

criterion_group!(benches, bench_10_xyz_rapids);
criterion_main!(benches);
