#[macro_use]
extern crate criterion;

extern crate lifelib;

use criterion::Criterion;



fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("lifelib::bench", |b| b.iter(|| lifelib::bench()));
    c.bench_function("lifelib::ndbench", |b| b.iter(|| lifelib::ndbench()));    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
