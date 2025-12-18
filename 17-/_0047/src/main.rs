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
    let consecutive: u64 = 4;
    let mut all_divisors = VecDeque::new();
    for num in 1..1_000_000 {
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
        all_divisors.push_back(divisors.len() as u64);
        if all_divisors.len() as u64 >= consecutive {
            let n_divisors = all_divisors.iter().min().unwrap().clone();
            all_divisors.pop_front();
            if n_divisors < consecutive {
                continue;
            }
            res = num - consecutive + 1;
            break;
        }
    }
    println!("Res: {res}");
}
