use pe_lib::primes::find_primes_sieve;
use std::collections::HashMap;
use std::time::Instant;

fn break_into_primes(n: u64, primes: &Vec<u64>, highest: u64, memo: &mut HashMap<(u64, u64), u64>) {
    if memo.get(&(n, highest)).is_some() {
        return;
    }
    let mut res = 0;
    if n == 0 {
        res = 1;
    } else {
        for p in primes {
            if *p > highest || *p > n {
                break;
            }
            break_into_primes(n - p, primes, *p, memo);
            res += memo.get(&(n - p, *p)).unwrap();
        }
    }
    memo.insert((n, highest), res);
}
fn main() {
    let start = Instant::now();
    let primes = find_primes_sieve(100);
    let mut memo = HashMap::new();
    let mut res = 0;
    for i in 1..1_000_000 {
        break_into_primes(i, &primes, i - 1, &mut memo);
        if memo[&(i, i - 1)] >= 5_000 {
            res = i;
            break;
        }
    }

    println!("Res: {res}");
    println!("Duration: {}micros", start.elapsed().as_micros());
}
