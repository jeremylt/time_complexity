# Time Complexity

This repository contains a basic demonstration of time complexity analysis.

## Live Link

An online demo can be seen on repl.com:
https://replit.com/@jeremylt/timecomplexity

Note, it takes a little bit of time for repl.com to build and run this repository.

## Building

This repository uses Rust. To build

    cargo build

To test

    cargo test

To benchmark

    cargo bench

The command `cargo bench` provides runtimes for all of the functions included in this crate.

## Initial Analysis

The function `is_prime_all` uses a prime checking algorithm with a time complexity of `O(n)`, so runtime should scale linearly with problem size.

The function `is_prime_half` uses a prime checking algorithm that also has a time complexity of `O(n)`, so runtime should scale linearly with problem size.

The function `is_prime_sqrt` use a prime checking algorithm with a time complexity of `O(n^1/2)`, so runtime should scale with the square root of problem size.

## Cumulative Effects

An algorithm with higher time complexity can have a cumulative effect if called frequently enough.

The function `sum_primes` accepts an upper bound and a prime checking function as arugments, so benchmarking this function helps demonstrate the cumulative effect of repeatedly calling our `sum_prime_*` functions with a different time complexities.

The function `sum_primes_seive` is also provided to demonstrate an algorithm with much better time complexity for finding all prime numbers in a range.
Interestingly, the prime number seive will only take twice as much time to find all primes from `2` to `num` as verifying that `num` is prime with `is_prime_sqrt`.
