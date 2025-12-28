use pe_lib::primes::find_primes_sieve;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let max_n = 1_000_000;
    let primes = find_primes_sieve(max_n);
    let mut map: HashMap<u64, HashSet<u64>> = HashMap::new();
    for n in 2..max_n {
        let sqrt = (n as f64).sqrt() as u64;
        let mut not_primes = HashSet::new();
        for p in primes.iter() {
            if *p > sqrt {
                break;
            }
            if n % p == 0 {
                let entry = map.entry(*p).or_insert(HashSet::new());
                if entry.len() > 0 {
                    not_primes = not_primes.union(entry).cloned().collect()
                }
                entry.insert(n);
            }
        }
    }
    println!("Duration: {}ms", start.elapsed().as_millis())
}
