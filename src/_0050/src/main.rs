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
    let mut max_n = 0;
    let mut max_p = 0;
    'outer: for i in 0..primes.len() {
        let mut cumsum = 0;
        for j in (0..=i).rev() {
            cumsum += primes[j];
            if cumsum > 1_000_000 {
                continue 'outer;
            }
            let n = i - j + 1;
            if n > max_n && primes.binary_search(&cumsum).is_ok() {
                max_p = cumsum;
                max_n = n;
            }
        }
    }
    println!("Res: {max_p}");
}
