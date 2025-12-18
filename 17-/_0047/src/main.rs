use std::cmp::min;
use std::collections::{HashSet, VecDeque};

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
    find_primes(1_000_000, &mut primes);
    let mut res = 0;
    let consecutive = 3;
    let mut all_divisors = VecDeque::new();
    for num in 1..1_000_000 {
        if num == 646 {
            println!("debug");
        }
        let mut divisors = HashSet::new();
        for j in 0..primes.len() {
            let p = primes[j];
            if p > num {
                break;
            }
            if num % p == 0 {
                divisors.insert(p);
            }
        }
        all_divisors.push_back(divisors);
        if all_divisors.len() >= consecutive {
            let mut n_divisors = 2;
            for d in all_divisors.iter() {
                n_divisors = min(n_divisors, d.len());
            }
            let mut intersection = all_divisors[0].clone();
            for j in 1..consecutive {
                intersection = intersection
                    .intersection(&all_divisors[j])
                    .cloned()
                    .collect();
            }
            all_divisors.pop_front();
            if n_divisors < 2 {
                continue;
            }
            if intersection.len() != 0 {
                res = num;
                break;
            }
        }
    }
    println!("Res: {res}");
}
