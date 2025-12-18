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

fn get_prime_fact(mut n: i32, primes: &Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; primes.len()];
    for i in 0..primes.len() {
        let p = primes[i];
        while n % p == 0 {
            n = n / p;
            res[i] += 1;
        }
    }
    return res;
}
fn main() {
    let n = 100;
    let mut primes = vec![2];
    find_primes(100, &mut primes);
    let mut memo = HashSet::new();
    for a in 2..=n {
        let prime_fact = get_prime_fact(a, &primes);
        let mut temp = vec![0; prime_fact.len()];
        for b in 2..=n {
            for (i, x) in prime_fact.iter().enumerate() {
                temp[i] = x * b;
            }
            memo.insert(temp.clone());
        }
    }
    println!("Res {}", memo.len());
}
