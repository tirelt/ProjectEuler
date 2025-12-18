fn find_primes(n: u64, primes: &mut Vec<u64>) {
    'outer: for num in (1 + primes.last().unwrap())..=n {
        let sqrt = (num as f64).sqrt() as u64 + 1;
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
    find_primes(10_000_000, &mut primes);
    let mut res = 0;
    'pair: for i in 1..1_000_000 {
        let num: u64 = 2 * i + 1;
        if !primes.contains(&num) {
            for i in 0..primes.len() {
                if primes[i] > num {
                    res = num;
                    break 'pair;
                }
                let rest = (num - primes[i]) / 2;
                let sqrt = (rest as f64).sqrt() as u64;
                if sqrt * sqrt == rest {
                    continue 'pair;
                }
            }
        }
    }
    println!("Res: {res}");
}
