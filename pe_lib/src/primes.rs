pub fn find_primes_sieve(n: u64) -> Vec<u64> {
    let n = n as usize;
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    is_prime[2] = true;

    for i in (3..=n).step_by(2) {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i * 2) {
                is_prime[j] = false;
            }
        }
        if (i as u64) * (i as u64) > n as u64 {
            break;
        }
    }

    let mut primes = vec![2];
    for i in (3..=n).step_by(2) {
        if is_prime[i] {
            primes.push(i as u64);
        }
    }
    primes
}
pub fn find_primes(n: u64, primes: &mut Vec<u64>) {
    if primes.len() == 0 {
        primes.push(2);
    }
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

fn pow_mod(mut a: u64, mut n: u64, m: u64) -> u64 {
    // computes a^n mod m
    let mut res: u64 = 1;
    a %= m;
    while n > 0 {
        //bitwise AND to check the last bit of n and hence its parity
        if n & 1 == 1 {
            res = res.wrapping_mul(a) % m;
        }
        a = a.wrapping_mul(a) % m;
        // bitwise right shift  by 1 to divide by 2
        n >>= 1;
    }
    res
}
fn witness(a: u64, n: u64, d: u64, s: u64) -> bool {
    let mut value = pow_mod(a, d, n);
    if value == 1 || value == n - 1 {
        return true;
    }
    for _ in 1..s {
        value = (value * value) % n;
        if value == n - 1 {
            return true;
        }
    }
    false
}
pub fn is_prime_mr(n: u64) -> bool {
    if n <= 2 || n & 1 == 0 {
        return n == 2; // only 2 is prime among evens and <2
    }
    if n >= 1u64 << 32 {
        panic!(
            "The algo is deterministic for numbers < 4,759,123,141 but our implementation using u64 restricts for numbers < 2^32"
        );
    }
    // Check for small primes
    if n < 1000 {
        for p in [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
        ]
        .iter()
        {
            if n == *p {
                return true;
            }
            if n % p == 0 {
                return false;
            }
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d & 1 == 0 {
        s += 1;
        d >>= 1;
    }
    witness(2, n, d, s) && witness(7, n, d, s) && witness(61, n, d, s)
}

pub fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (0, y) => y,
        (x, y) if x > y => gcd(y, x),
        (x, y) => gcd(y % x, x),
    }
}
