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
    let mut primes = vec![2 as u64];
    let mut curr: u64 = 9;
    let mut n_primes = 3;
    let mut n_numbers = 5;
    let mut counter = 1;
    while n_primes as f64 / n_numbers as f64 > 0.1 {
        counter += 1;
        curr += 8 * counter;
        let new_numbers: Vec<u64> = (0..=3).map(|x| curr - (2 * x) * counter).collect();
        find_primes(curr, &mut primes);
        for new_n in new_numbers {
            if primes.binary_search(&new_n).is_ok() {
                n_primes += 1;
            }
        }
        n_numbers += 4;
        println!(
            "curr: {curr} - Counter: {counter} - %{:.2}",
            n_primes as f64 / n_numbers as f64
        );
    }
    let size = counter * 2 + 1;
    println!("Res: {size}");
}
