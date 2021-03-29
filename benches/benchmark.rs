use criterion::{black_box, criterion_group, criterion_main, Criterion};
use time_complexity::{is_prime, is_prime_all, is_prime_half, is_prime_sqrt, sum_primes};

// ----------------------------------------------------------------------------
// Test sizes
// ----------------------------------------------------------------------------
const SMALL: i64 = 1_789;
const LARGE: i64 = 17_903;

// ----------------------------------------------------------------------------
// Benchmark all divisors
// ----------------------------------------------------------------------------
pub fn all_small_benchmark(c: &mut Criterion) {
    c.bench_function("all 1789", |b| b.iter(|| is_prime_all(black_box(SMALL))));
}

pub fn all_large_benchmark(c: &mut Criterion) {
    c.bench_function("all 17903", |b| b.iter(|| is_prime_all(black_box(LARGE))));
}

pub fn sum_primes_all_small_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes_all 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime_all))
    });
}

pub fn sum_primes_all_large_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes_all 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime_all))
    });
}

// ----------------------------------------------------------------------------
// Benchmark half divisors
// ----------------------------------------------------------------------------
pub fn half_small_benchmark(c: &mut Criterion) {
    c.bench_function("half 1789", |b| b.iter(|| is_prime_half(black_box(SMALL))));
}

pub fn half_large_benchmark(c: &mut Criterion) {
    c.bench_function("half 17903", |b| b.iter(|| is_prime_half(black_box(LARGE))));
}

pub fn sum_primes_half_small_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes_half 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime_half))
    });
}

pub fn sum_primes_half_large_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes_half 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime_half))
    });
}

// ----------------------------------------------------------------------------
// Benchmark sqrt divisors
// ----------------------------------------------------------------------------
pub fn sqrt_small_benchmark(c: &mut Criterion) {
    c.bench_function("sqrt 1789", |b| b.iter(|| is_prime_sqrt(black_box(SMALL))));
}

pub fn sqrt_large_benchmark(c: &mut Criterion) {
    c.bench_function("sqrt 17903", |b| b.iter(|| is_prime_sqrt(black_box(LARGE))));
}

pub fn sum_primes_sqrt_small_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes_sqrt 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime_sqrt))
    });
}

pub fn sum_primes_sqrt_large_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes_sqrt 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime_sqrt))
    });
}

// ----------------------------------------------------------------------------
// Benchmark idiomatic form
// ----------------------------------------------------------------------------
pub fn idiomatic_small_benchmark(c: &mut Criterion) {
    c.bench_function("idiomatic 1789", |b| b.iter(|| is_prime(black_box(SMALL))));
}

pub fn idiomatic_large_benchmark(c: &mut Criterion) {
    c.bench_function("idiomatic 17903", |b| b.iter(|| is_prime(black_box(LARGE))));
}

pub fn sum_primes_idiomatic_small_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime))
    });
}

pub fn sum_primes_idiomatic_large_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime))
    });
}

// ----------------------------------------------------------------------------

criterion_group!(
    benches,
    all_small_benchmark,
    all_large_benchmark,
    sum_primes_all_small_benchmark,
    sum_primes_all_large_benchmark,
    half_small_benchmark,
    half_large_benchmark,
    sum_primes_half_small_benchmark,
    sum_primes_half_large_benchmark,
    sqrt_small_benchmark,
    sqrt_large_benchmark,
    sum_primes_sqrt_small_benchmark,
    sum_primes_sqrt_large_benchmark,
    idiomatic_small_benchmark,
    idiomatic_large_benchmark,
    sum_primes_idiomatic_small_benchmark,
    sum_primes_idiomatic_large_benchmark,
);
criterion_main!(benches);

// ----------------------------------------------------------------------------
