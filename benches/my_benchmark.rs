#[macro_use]
extern crate criterion;


extern crate lifelib;

use criterion::Criterion;



fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lifelib::bench", |b| b.iter(|| lifelib::bench()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
