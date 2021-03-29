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
