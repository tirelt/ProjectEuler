use pe_lib::primes::find_primes_sieve;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let max_n = 1_000_000;
    let primes = find_primes_sieve(max_n);
    let mut res = 0;
    let mut max_ratio = 0.0;
    for n in 2..=max_n {
        let mut primes_divisor = Vec::new();
        let mut n_not_primes = 0;
        if primes.binary_search(&n).is_err() {
            let mut n_ = n;
            for p in primes.iter() {
                if n % p == 0 {
                    n_not_primes += (n - 1) / p;
                    for c in primes_divisor.iter() {
                        n_not_primes -= (n - 1) / (*c * p);
                    }
                    primes_divisor.push(*p);
                    while n_ % p == 0 {
                        n_ /= p;
                    }
                    if n_ == 1 {
                        break;
                    }
                }
            }
        }
        let n_primes = n - 1 - n_not_primes;
        let ratio = n as f64 / n_primes as f64;
        println!(
            "n={n}, ratio={ratio:.2}, phi(n)={n_primes}, primes_divisors={:?}",
            primes_divisor
        );
        if ratio > max_ratio {
            res = n;
            max_ratio = ratio;
        }
    }
    println!("Res: {res}");
    println!("Duration: {}ms", start.elapsed().as_millis())
}
