use criterion::{black_box, criterion_group, criterion_main, Criterion};
use time_complexity::{
    is_prime, is_prime_all, is_prime_half, is_prime_sqrt, sum_primes, sum_primes_seive,
};

// ----------------------------------------------------------------------------
// Test sizes
// ----------------------------------------------------------------------------
const SMALL: i64 = 1_789;
const LARGE: i64 = 17_903;

// ----------------------------------------------------------------------------
// Benchmark all divisors
// ----------------------------------------------------------------------------
pub fn is_prime_all_small_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime_all 1789", |b| {
        b.iter(|| is_prime_all(black_box(SMALL)))
    });
}

pub fn is_prime_all_large_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime_all 17903", |b| {
        b.iter(|| is_prime_all(black_box(LARGE)))
    });
}

pub fn sum_primes_all_small_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes all 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime_all))
    });
}

pub fn sum_primes_all_large_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes all 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime_all))
    });
}

// ----------------------------------------------------------------------------
// Benchmark half divisors
// ----------------------------------------------------------------------------
pub fn is_prime_half_small_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime_half 1789", |b| {
        b.iter(|| is_prime_half(black_box(SMALL)))
    });
}

pub fn is_prime_half_large_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime_half 17903", |b| {
        b.iter(|| is_prime_half(black_box(LARGE)))
    });
}

pub fn sum_primes_half_small_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes half 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime_half))
    });
}

pub fn sum_primes_half_large_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes half 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime_half))
    });
}

// ----------------------------------------------------------------------------
// Benchmark sqrt divisors
// ----------------------------------------------------------------------------
pub fn is_prime_sqrt_small_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime_sqrt 1789", |b| {
        b.iter(|| is_prime_sqrt(black_box(SMALL)))
    });
}

pub fn is_prime_sqrt_large_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime_sqrt 17903", |b| {
        b.iter(|| is_prime_sqrt(black_box(LARGE)))
    });
}

pub fn sum_primes_sqrt_small_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes sqrt 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime_sqrt))
    });
}

pub fn sum_primes_sqrt_large_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes sqrt 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime_sqrt))
    });
}

// ----------------------------------------------------------------------------
// Benchmark idiomatic form
// ----------------------------------------------------------------------------
pub fn is_prime_idiomatic_small_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime_idiomatic 1789", |b| {
        b.iter(|| is_prime(black_box(SMALL)))
    });
}

pub fn is_prime_idiomatic_large_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime_idiomatic 17903", |b| {
        b.iter(|| is_prime(black_box(LARGE)))
    });
}

pub fn sum_primes_idiomatic_small_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes idiomatic 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime))
    });
}

pub fn sum_primes_idiomatic_large_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes idiomatic 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime))
    });
}

// ----------------------------------------------------------------------------
// Benchmark prime number seive
// ----------------------------------------------------------------------------
pub fn sum_primes_seive_small_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes_seive 1789", |b| {
        b.iter(|| sum_primes_seive(black_box(SMALL)))
    });
}

pub fn sum_primes_seive_large_benchmark(c: &mut Criterion) {
    c.bench_function("sum_primes_seive 17903", |b| {
        b.iter(|| sum_primes_seive(black_box(LARGE)))
    });
}

// ----------------------------------------------------------------------------

criterion_group!(
    benches,
    is_prime_all_small_benchmark,
    is_prime_all_large_benchmark,
    sum_primes_all_small_benchmark,
    sum_primes_all_large_benchmark,
    is_prime_half_small_benchmark,
    is_prime_half_large_benchmark,
    sum_primes_half_small_benchmark,
    sum_primes_half_large_benchmark,
    is_prime_sqrt_small_benchmark,
    is_prime_sqrt_large_benchmark,
    sum_primes_sqrt_small_benchmark,
    sum_primes_sqrt_large_benchmark,
    is_prime_idiomatic_small_benchmark,
    is_prime_idiomatic_large_benchmark,
    sum_primes_idiomatic_small_benchmark,
    sum_primes_idiomatic_large_benchmark,
    sum_primes_seive_small_benchmark,
    sum_primes_seive_large_benchmark,
);
criterion_main!(benches);

// ----------------------------------------------------------------------------
