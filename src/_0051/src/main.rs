use std::collections::BTreeMap;

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

fn set_p(mut p: i64, old: i64, new: i64, posistions: &Vec<u32>) -> u64 {
    for pos in posistions {
        p += (new - old) * (10 as i64).pow(*pos);
    }
    return p as u64;
}
fn main() {
    let mut primes = vec![2];
    find_primes(1_000_000, &mut primes);
    let mut res = 0;
    let n_target = 8;
    'outer: for p in primes.iter() {
        let mut digits = BTreeMap::new();
        let mut p_ = p.clone();
        let mut counter: u32 = 0;
        while p_ > 0 {
            digits
                .entry(p_ % 10 as u64)
                .or_insert(Vec::new())
                .push(counter);
            counter += 1;
            p_ /= 10;
        }
        for (d, pos) in digits {
            let start = if pos.contains(&(counter - 1)) { 1 } else { 0 };
            let mut n = 0;
            for i in start..=9 {
                let p_ = set_p(*p as i64, d as i64, i, &pos);
                if primes.binary_search(&p_).is_ok() {
                    n += 1;
                }
            }
            if n >= n_target {
                res = p.clone();
                break 'outer;
            }
        }
    }
    println!("Res: {res}");
}
