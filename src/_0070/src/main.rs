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
fn compute_phi(n: u64, curr_primes: &Vec<u64>) -> u64 {
    let mut phi = 0;
    for i in 0..curr_primes.len() {
        phi += (n - 1) / curr_primes[i];
        for j in (i + 1)..curr_primes.len() {
            phi -= (n - 1) / (curr_primes[i] * curr_primes[j]);
        }
    }
    phi
}
fn generate_composite(
    primes: &Vec<u64>,
    curr_n: u64,
    curr_primes: &mut Vec<u64>,
    curr_i: usize,
    res: &mut Vec<(f64, u64)>,
) {
    for i in curr_i..primes.len() {
        let p = primes[i];
        let n = p * curr_n;
        if n < 10_000_000 {
            curr_primes.push(p);
            // check
            let phi = compute_phi(n, curr_primes);
            if check_digits(phi) == check_digits(n) {
                res.push((n as f64 / phi as f64, n));
            }
            // test futher
            generate_composite(primes, n, curr_primes, curr_i + 1, res);
            // backtrack
            curr_primes.pop();
        }
    }
}
fn main() {
    let start = Instant::now();
    let max_n = 10_000_000;
    let primes = find_primes_sieve(max_n);
    let mut curr_primes = Vec::new();
    let mut res_all: Vec<(f64, u64)> = Vec::new();
    generate_composite(&primes, 1, &mut curr_primes, 0, &mut res_all);
    //res_all.sort_by(|&(a, b), &(x, y)| a.cmp(&x));
    println!("N primes  {}", primes.len());
    println!("Duration: {}ms", start.elapsed().as_millis())
}
