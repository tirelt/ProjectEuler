use pe_lib::primes::find_primes_sieve;
fn main() {
    let primes = find_primes_sieve(1_000);
    let mut res = (0, 0, 0);
    for d in 111..=1_000 as i128 {
        let sqrt = (d as f64).sqrt() as i128;
        if sqrt.pow(2) == d {
            continue;
        }
        let d_is_prime = primes.contains(&(d as u64));
        let mut x: i128 = 1;
        let mut y: i128 = 1;
        let mut i = 1;
        while x.pow(2) - d * y.pow(2) != 1 {
            while x.pow(2) - d * y.pow(2) > 1 {
                y += 1;
            }
            if x.pow(2) - d * y.pow(2) == 1 {
                break;
            }
            i += 1;
            if d_is_prime {
                x = d * (i / 2);
                if i % 2 == 0 {
                    x -= 1;
                } else {
                    x += 1;
                }
            } else {
                x += 1;
            }
            y = ((x.pow(2) - 1) as f64 / d as f64).sqrt() as i128;
            if y == 0 {
                y = 1;
            }
        }
        println!("solved: {:?}", (d, x, y));
        if x > res.0 {
            res = (d, x, y);
        }
    }
    println!("Res: {:?}", res);
}
