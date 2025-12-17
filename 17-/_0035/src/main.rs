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

fn generate_rotation(mut n: i32) -> Vec<i32> {
    let mut rotations = Vec::new();
    let n_digits = (n as f64).log10() as u32;
    let p = (10 as i32).pow(n_digits);
    for _i in 0..=n_digits {
        let rest = n % p;
        let last = n / p;
        n = rest * 10 + last;
        rotations.push(n);
    }
    return rotations;
}

fn main() {
    let mut primes = vec![2];
    let mut res = 0;
    find_primes(1_000_000, &mut primes);
    'outer: for p in primes.iter() {
        let rotations = generate_rotation(*p);
        for r in rotations {
            if !primes.contains(&r) {
                continue 'outer;
            }
        }
        res += 1;
    }
    println!("Res: {res}");
}
