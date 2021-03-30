# Time Complexity

This repository contains a basic demonstration of time complexity analysis.

The function `is_prime_all` uses a prime checking algorithm with a time complexity of `O(n)`, so runtime should scale linearly with problem size.

The function `is_prime_half` uses a prime checking algorithm that also has a time complexity of `O(n)`, so runtime should scale linearly with problem size.

The function `is_prime_sqrt` use a prime checking algorithm with a time complexity of `O(n^1/2)`, so runtime should scale with the square root of problem size.

## Building

This repository uses Rust. To build

    cargo build

To test

    cargo test

To benchmark

    cargo bench

## Cumulative Effects

An algorithm with higher time complexity can have a cumulative effect if called frequently enough.

The function `sum_primes` accepts an upper bound and a prime checking function as arugments, so benchmarking this function helps demonstrate the cumulative effect of repeatedly calling this function with a higher time complexity.

The function `sum_primes_seive` is also provided to demonstrate an algorithm with much better time complexity for finding all prime numbers in a range.

## Live Link

An online demo can be seen on repl.it:
https://replit.com/@jeremylt/timecomplexity
