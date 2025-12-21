use pe_lib::primes::find_primes_sieve;
use std::time::Instant;

fn check(p_1: u64, p_2: u64, primes: &Vec<u64>) -> bool {
    let p_1 = p_1.to_string();
    let p_2 = p_2.to_string();
    let mut test_1 = p_1.clone();
    test_1.push_str(&p_2);
    let test_1: u64 = test_1.parse().unwrap();
    if !primes.binary_search(&test_1).is_ok() {
        return false;
    }
    let mut test_2 = p_2.clone();
    test_2.push_str(&p_1);
    let test_2: u64 = test_2.parse().unwrap();
    primes.binary_search(&test_2).is_ok()
}
fn main() {
    let now = Instant::now();
    let primes = find_primes_sieve(100_000_000);
    let max_p = 10_000;
    let mut max_p_idx = 0;
    while primes[max_p_idx] < max_p {
        max_p_idx += 1;
    }
    let mut res = 1 << 32;
    for i in 0..max_p_idx {
        let p_i = primes[i];
        for j in (i + 1)..max_p_idx {
            let p_j = primes[j];
            if check(p_i, p_j, &primes) {
                for k in (j + 1)..max_p_idx {
                    let curr_p = vec![p_i, p_j];
                    let p_k = primes[k];
                    if curr_p
                        .iter()
                        .fold(true, |acc, p| acc && check(*p, p_k, &primes))
                    {
                        for l in (k + 1)..max_p_idx {
                            let curr_p = vec![p_i, p_j, p_k];
                            let p_l = primes[l];
                            if curr_p
                                .iter()
                                .fold(true, |acc, p| acc && check(*p, p_l, &primes))
                            {
                                for n in (l + 1)..max_p_idx {
                                    let curr_p = vec![p_i, p_j, p_k, p_l];
                                    let p_n = primes[n];
                                    let mut sum: u64 = curr_p.iter().sum();
                                    sum += p_n;
                                    if sum < res
                                        && curr_p
                                            .iter()
                                            .fold(true, |acc, p| acc && check(*p, p_n, &primes))
                                    {
                                        println!("{p_i}-{p_j}-{p_k}-{p_l}-{p_n}");
                                        res = sum;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}ms", now.elapsed().as_millis());
    println!("{res}");
}
