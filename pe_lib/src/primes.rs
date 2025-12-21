pub fn find_primes(n: u64, primes: &mut Vec<u64>) {
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

pub fn is_prime_from_primes(n: u64, primes: &mut Vec<u64>) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    find_primes(sqrt, primes);
    for p in primes.iter() {
        if *p > sqrt {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    true
}

pub fn is_prime_sqrt(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
