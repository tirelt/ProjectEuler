use pe_lib::primes::find_primes_sieve;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let max_n = 10_000_000;
    let primes = find_primes_sieve(max_n);
    println!("Duration: {}ms", start.elapsed().as_millis())
}
