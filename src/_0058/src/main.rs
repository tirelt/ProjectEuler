use pe_lib::primes::is_prime_from_primes;
//use pe_lib::primes::is_prime_sqrt;
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut primes = vec![2 as u64];
    let mut curr: u64 = 9;
    let mut n_primes = 3;
    let mut n_numbers = 5;
    let mut counter = 1;
    while n_primes as f64 / n_numbers as f64 > 0.1 {
        counter += 1;
        curr += 8 * counter;
        let new_numbers: Vec<u64> = (0..=3).map(|x| curr - (2 * x) * counter).collect();
        for new_n in new_numbers {
            /*
            if is_prime_sqrt(new_n) {
                n_primes += 1;
            }
            */
            if is_prime_from_primes(new_n, &mut primes) {
                n_primes += 1;
            }
        }
        n_numbers += 4;
    }
    let size = counter * 2 + 1;
    println!("Res: {size}");
    println!("{}ms", now.elapsed().as_millis());
}
