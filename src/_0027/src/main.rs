fn find_primes(n: i32, primes: &mut Vec<i32>) {
    'outer: for num in (1 + primes.last().unwrap())..=n {
        let sqrt = (num as f64).sqrt() as i32 + 1;
        for p in primes.iter() {
            if num % *p == 0 {
                continue 'outer;
            }
            if *p > sqrt {
                break;
            }
        }
        primes.push(num);
    }
}
fn main() {
    let mut primes = vec![2];
    find_primes(1000, &mut primes);
    let b_values = primes.clone();
    let mut res = 0;
    let mut max = 0;
    for b in b_values {
        //let min_a = -2 * ((b as f64).sqrt() as i32) - 1;
        for a in -999..1000 {
            let mut n = 0;
            let calc = |x| x * x + a * x + b;
            let mut vals = Vec::new();
            loop {
                let val: i32 = calc(n);
                if val < 0 {
                    break;
                }
                vals.push(val.clone());
                if val > *primes.last().unwrap() {
                    find_primes(val * 2, &mut primes);
                }
                if !primes.contains(&val) {
                    break;
                }
                n += 1;
            }
            if n > max {
                max = n;
                res = b * a;
            }
        }
    }
    println!("Res: {res} - n = {max}");
}
