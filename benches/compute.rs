#![feature(generic_const_exprs)]

use church::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("product", |b| {
        b.iter(|| PRODUCT.call(black_box(&[500, 100])))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
