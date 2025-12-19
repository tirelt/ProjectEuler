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
}
