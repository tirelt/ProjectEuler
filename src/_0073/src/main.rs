use pe_lib::primes::gcd;
use std::time::Instant;

fn main() {
    let start_t = Instant::now();
    let n_start = 1;
    let d_start = 3;
    let start = n_start as f64 / d_start as f64;
    let n_end = 1;
    let d_end = 2;
    let end = n_end as f64 / d_end as f64;
    let max_d = 12_000;
    let mut counter = 0;
    for i in 2..=max_d {
        let new_n_end = i * n_end / d_end;
        let new_n_start = i * n_start / d_start + 1;
        for n in new_n_start..=new_n_end {
            let val = n as f64 / i as f64;
            if val < end && val > start && gcd(n, i) == 1 {
                counter += 1;
            }
        }
    }
    println!("Res: {:?}", counter);
    println!("Duration: {}ms", start_t.elapsed().as_millis());
}
