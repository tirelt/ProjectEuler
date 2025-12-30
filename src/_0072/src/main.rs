use pe_lib::primes::find_primes_sieve;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let max_n = 1_000_000;
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
    //let mut primes_divisor = &Vec::new();
    for n in 2..=max_n {
        let mut phi = n;
        if primes.binary_search(&n).is_err() {
            let primes_divisor = &primes_divisors_sieve[n as usize];
            for p in primes_divisor {
                phi = phi - phi / p;
            }
        } else {
            phi -= 1;
        }
        res += phi;
    }
    println!("Res: {res}");
    println!("Duration: {}ms", start.elapsed().as_millis());
}
