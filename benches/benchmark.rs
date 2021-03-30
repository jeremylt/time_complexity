use criterion::{black_box, criterion_group, criterion_main, Criterion};
use time_complexity::{
    is_prime, is_prime_all, is_prime_half, is_prime_sqrt, sum_primes, sum_primes_sieve,
};

// ----------------------------------------------------------------------------
// Test sizes
// ----------------------------------------------------------------------------
const SMALL: i64 = 1_789;
const LARGE: i64 = 17_903;

// ----------------------------------------------------------------------------
// Benchmark all divisors
// ----------------------------------------------------------------------------
pub fn is_prime_all_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("all divisors");

    group.bench_function("is_prime_all 1789", |b| {
        b.iter(|| is_prime_all(black_box(SMALL)))
    });
    group.bench_function("is_prime_all 17903", |b| {
        b.iter(|| is_prime_all(black_box(LARGE)))
    });
}

pub fn sum_primes_all_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum primes - all divisors");

    group.bench_function("sum_primes all 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime_all))
    });
    group.bench_function("sum_primes all 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime_all))
    });
}

// ----------------------------------------------------------------------------
// Benchmark half divisors
// ----------------------------------------------------------------------------
pub fn is_prime_half_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("half divisors");

    group.bench_function("is_prime_half 1789", |b| {
        b.iter(|| is_prime_half(black_box(SMALL)))
    });
    group.bench_function("is_prime_half 17903", |b| {
        b.iter(|| is_prime_half(black_box(LARGE)))
    });
}

pub fn sum_primes_half_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum primes - half divisors");

    group.bench_function("sum_primes half 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime_half))
    });
    group.bench_function("sum_primes half 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime_half))
    });
}

// ----------------------------------------------------------------------------
// Benchmark sqrt divisors
// ----------------------------------------------------------------------------
pub fn is_prime_sqrt_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqrt divisors");

    group.bench_function("is_prime_sqrt 1789", |b| {
        b.iter(|| is_prime_sqrt(black_box(SMALL)))
    });
    group.bench_function("is_prime_sqrt 17903", |b| {
        b.iter(|| is_prime_sqrt(black_box(LARGE)))
    });
}

pub fn sum_primes_sqrt_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum primes - sqrt divisors");

    group.bench_function("sum_primes sqrt 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime_sqrt))
    });
    group.bench_function("sum_primes sqrt 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime_sqrt))
    });
}

// ----------------------------------------------------------------------------
// Benchmark idiomatic form
// ----------------------------------------------------------------------------
pub fn is_prime_idiomatic_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ideomatic form");

    group.bench_function("is_prime_idiomatic 1789", |b| {
        b.iter(|| is_prime(black_box(SMALL)))
    });
    group.bench_function("is_prime_idiomatic 17903", |b| {
        b.iter(|| is_prime(black_box(LARGE)))
    });
}

pub fn sum_primes_idiomatic_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum primes - ideomatic form");

    group.bench_function("sum_primes idiomatic 1789", |b| {
        b.iter(|| sum_primes(black_box(SMALL), is_prime))
    });
    group.bench_function("sum_primes idiomatic 17903", |b| {
        b.iter(|| sum_primes(black_box(LARGE), is_prime))
    });
}

// ----------------------------------------------------------------------------
// Benchmark prime number sieve
// ----------------------------------------------------------------------------
pub fn sum_primes_sieve_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum primes - sieve");

    group.bench_function("sum_primes_sieve 1789", |b| {
        b.iter(|| sum_primes_sieve(black_box(SMALL)))
    });
    group.bench_function("sum_primes_sieve 17903", |b| {
        b.iter(|| sum_primes_sieve(black_box(LARGE)))
    });
}

// ----------------------------------------------------------------------------

criterion_group!(
    benches,
    is_prime_all_benchmark,
    sum_primes_all_benchmark,
    is_prime_half_benchmark,
    sum_primes_half_benchmark,
    is_prime_sqrt_benchmark,
    sum_primes_sqrt_benchmark,
    is_prime_idiomatic_benchmark,
    sum_primes_idiomatic_benchmark,
    sum_primes_sieve_benchmark,
);
criterion_main!(benches);

// ----------------------------------------------------------------------------
