use std::collections::HashSet;

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
    /* cannot be 9 or 8 because 8/9 pandigital numbes are divisible by 9 */
    let mut primes = vec![2];
    find_primes(10_000_000, &mut primes);
    let mut res = 0;
    'outer: for p in primes.iter().rev() {
        let mut memo = HashSet::new();
        let n_digit = (*p as f64).log10() as i32 + 1;
        let mut p_ = p.clone();
        while p_ > 0 {
            let digit = p_ % 10;
            if digit == 0 || digit > n_digit || !memo.insert(digit) {
                continue 'outer;
            }
            p_ = p_ / 10;
        }
        if memo.get(&0).is_some() {
            continue;
        }
        res = *p;
        break;
    }
    println!("Res: {res}");
}
