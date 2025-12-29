use pe_lib::primes::gcd;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let n = 3;
    let d = 7;
    let max_d = 1_000_000;
    let mut res = (0.0, 0, 0);
    for i in 2..=max_d {
        let new_n = i * n / d;
        let c = gcd(new_n, i);
        let i = i / c;
        let new_n = new_n / c;
        if (new_n, i) != (n, d) {
            let val = new_n as f64 / i as f64;
            if val > res.0 {
                res = (val, new_n, i);
            }
        }
    }
    println!("Res: {:?}", res);
    println!("Duration: {}ms", start.elapsed().as_millis());
}
