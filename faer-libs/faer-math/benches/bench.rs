use criterion::{criterion_group, criterion_main, Criterion};
// use faer_math::{};
use std::time::Duration;

use dyn_stack::*;
use rand::random;

use faer_core::{Mat, Parallelism};

criterion_group!(
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(5))
        .sample_size(10);
    targets = bidiag, bidiag_svd, real_svd,
);
criterion_main!(benches);
