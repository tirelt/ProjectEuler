use std::collections::BTreeMap;

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

fn generate_permutation(
    curr: u64,
    remaining: BTreeMap<u64, u64>,
    res: &mut Vec<u64>,
    primes: &Vec<u64>,
) {
    if remaining.len() == 0 && primes.contains(&curr) {
        res.push(curr);
    }
    for (n, occ) in remaining.iter() {
        let mut new_remaining = remaining.clone();
        if *occ == 1 {
            new_remaining.remove(n);
        } else {
            new_remaining.insert(n.clone(), *occ - 1);
        }
        generate_permutation(curr * 10 + *n, new_remaining, res, primes);
    }
}
fn main() {
    let mut primes = vec![2];

    find_primes(10_000, &mut primes);
    let primes: Vec<u64> = primes.into_iter().filter(|x| *x > 999).collect();
    let mut sequences = Vec::new();
    for p in primes.iter() {
        let mut p_ = p.clone();
        let mut remaining = BTreeMap::new();
        while p_ > 0 {
            *remaining.entry(p_ % 10).or_insert(0) += 1;
            p_ /= 10;
        }
        let mut permutations = Vec::new();
        generate_permutation(0, remaining, &mut permutations, &primes);
        permutations.sort();
        let i = permutations.iter().position(|x| x == p).unwrap();
        for j in (i + 1)..permutations.len() {
            for k in (j + 1)..permutations.len() {
                if permutations[j] - p == permutations[k] - permutations[j] {
                    sequences.push((p, permutations[j], permutations[k]));
                }
            }
        }
    }
    let last_seq = sequences.last().unwrap();
    let res = last_seq.0 * 100_000_000 + last_seq.1 * 10_000 + last_seq.2;
    println!("Res: {res}");
}
