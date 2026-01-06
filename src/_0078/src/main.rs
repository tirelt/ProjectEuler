use std::collections::HashMap;
use std::time::Instant;

fn break_into_primes(n: u128, highest: u128, memo: &mut HashMap<(u128, u128), u128>) {
    if memo.get(&(n, highest)).is_some() {
        return;
    }
    let mut res = 0;
    if n == 0 {
        res = 1;
    } else {
        let mut p = 1;
        while p <= highest && p <= n {
            break_into_primes(n - p, p, memo);
            res += memo.get(&(n - p, p)).unwrap();
            p += 1;
        }
    }
    memo.insert((n, highest), res);
}
fn main() {
    let start = Instant::now();
    let mut memo = HashMap::new();
    let mut res = 0;
    for i in 1..1_000_000 {
        break_into_primes(i, i, &mut memo);
        let temp_res = &memo[&(i, i)];
        if temp_res % 1_000_000 == 0 {
            res = i;
            break;
        }
    }
    println!("Res: {res}");
    println!("Duration: {}micros", start.elapsed().as_micros());
}
