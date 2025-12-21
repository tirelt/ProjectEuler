# Project Euler library 

Implements helper function that are useful to solve Project Euler problems.

This is a Rust package containing one library crate and several modules.

## Module: primes

### `is_prime_mr` 
Implements a deterministic [Miller Rabin](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test) algorithm using the result of [Jaeshke 1993](./doc/jaeschke.pdf) to only test bases $2,7,61$ to test the primality of numbers $< 2^{32}$.