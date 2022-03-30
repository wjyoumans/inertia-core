/// Comparison of [num_bigint], [rug], and [inertia_core] big integer multiplication.
/// Note [rug] returns incomplete computation values on operations with no owned rug Integers,
/// so we use `rug::Integer::from` to compute and assign the result.

use std::str::FromStr;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use num_bigint::BigInt;
use rug;
use inertia_core::Integer;

// 2^64
const S64: &'static str = "18446744073709551616";


#[allow(unused_must_use)]
fn bench_mul_num(n: &BigInt, m: &BigInt) {
    n * m;
}

#[allow(unused_must_use)]
fn bench_mul_rug(n: &rug::Integer, m: &rug::Integer) {
    rug::Integer::from(n * m);
}

#[allow(unused_must_use)]
fn bench_mul_inertia(n: &Integer, m: &Integer) {
    n * m;
}

fn bench_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("Integer-mul");
        
    let n_inertia = Integer::from(S64);
    let n_num = BigInt::from_str(S64).unwrap();
    let n_rug = rug::Integer::from_str(S64).unwrap();

    let mut m_num: BigInt;
    let mut m_rug: rug::Integer;
    let mut m_inertia: Integer;
    for &e in [1, 2, 4, 8].iter() {
        m_num = n_num.pow(e);
        group.bench_with_input(BenchmarkId::new("num", e), &m_num, |b, m_num| {
            b.iter(|| bench_mul_num(black_box(&n_num), black_box(&m_num)));
        });


        m_rug = rug::Integer::from(rug::ops::Pow::pow(&n_rug, e));
        group.bench_with_input(BenchmarkId::new("rug", e), &m_rug, |b, m_rug| {
            b.iter(|| bench_mul_rug(black_box(&n_rug), black_box(&m_rug)));
        });

        m_inertia = inertia_core::ops::Pow::pow(&n_inertia, e);
        group.bench_with_input(BenchmarkId::new("inertia", e), &m_inertia, |b, m_inertia| {
            b.iter(|| bench_mul_inertia(black_box(&n_inertia), black_box(&m_inertia)));
        });
    }
    group.finish();
}
criterion_group!(benches, bench_mul);
criterion_main!(benches);
