use pe_lib::primes::find_primes_sieve;
use std::time::Instant;

fn check_digits(mut n: u64) -> Vec<u64> {
    let mut n_digits = vec![0; 10];
    while n > 0 {
        n_digits[(n % 10) as usize] += 1;
        n /= 10;
    }
    n_digits
}

fn main() {
    let start = Instant::now();
    let max_n = 10_000_000;
    let primes = find_primes_sieve(max_n);
    println!("Duration sieves: {}ms", start.elapsed().as_millis());
    let mut primes_divisors_sieve = vec![Vec::new(); max_n as usize + 1];
    for p in primes.iter() {
        let mut n = p.clone();
        while n <= max_n {
            primes_divisors_sieve[n as usize].push(*p);
            n += p;
        }
    }
    println!("Duration primes sieves: {}ms", start.elapsed().as_millis());
    let mut res = 0;
    let mut max_ratio = 0.0;
    //let mut primes_divisor = &Vec::new();
    for n in 2..=max_n {
        let mut n_divisors = 0;
        if primes.binary_search(&n).is_err() {
            let primes_divisor = &primes_divisors_sieve[n as usize];
            for i in 0..primes_divisor.len() {
                n_divisors += (n - 1) / primes_divisor[i];
                for j in (i + 1)..primes_divisor.len() {
                    n_divisors -= (n - 1) / (primes_divisor[i] * primes_divisor[j]);
                }
            }
        }
        let phi = n - 1 - n_divisors;
        if check_digits(phi) == check_digits(n) {
            let ratio = n as f64 / phi as f64;
            if ratio > max_ratio {
                res = n;
                max_ratio = ratio;
            }
        }
        /*
        println!(
            "n={n}, ratio={ratio:.2}, phi(n)={phi}, primes_divisors={:?}",
            primes_divisor
        );
        */
    }
    println!("Res: {res}");
    println!("Duration: {}ms", start.elapsed().as_millis());
}
