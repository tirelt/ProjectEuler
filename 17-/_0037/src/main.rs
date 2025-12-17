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
    let mut primes = vec![2];
    find_primes(1_000_000, &mut primes);
    println!("Fround primes");
    let mut res_all = Vec::new();
    'outer: for p in primes.iter() {
        if *p > 10 {
            let p_str = p.to_string();
            for i in 1..p_str.len() {
                let right = p_str[i..p_str.len()].parse().unwrap();
                let left = p_str[0..(p_str.len() - i)].parse().unwrap();
                if !primes.contains(&right) || !primes.contains(&left) {
                    continue 'outer;
                }
            }
            res_all.push(p);
        }
    }
    println!("Numbers found: {} \n{res_all:?}", res_all.len());
    let res: i32 = res_all.into_iter().sum();
    println!("Res: {}", res);
}
