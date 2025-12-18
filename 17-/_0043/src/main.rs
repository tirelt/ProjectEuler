use std::collections::BTreeSet;

fn check(
    current: String,
    remaining: BTreeSet<u8>,
    mut divisor_index: usize,
    divisors: &Vec<u32>,
) -> u64 {
    if current.len() >= 10 {
        let n: u64 = current.parse().unwrap();
        return n;
    }
    divisor_index += if current.len() >= 3 { 1 } else { 0 };
    let mut res = 0;
    for el in remaining.iter() {
        let mut new_current = current.clone();
        new_current.push_str(&el.to_string());
        if new_current.len() >= 3 {
            let n: u32 = new_current[(new_current.len() - 3)..new_current.len()]
                .parse()
                .unwrap();
            if n % divisors[divisor_index] != 0 {
                continue;
            }
        }
        let mut new_remaining = remaining.clone();
        new_remaining.remove(el);
        res += check(new_current, new_remaining, divisor_index, divisors);
    }
    res
}
fn main() {
    let divisors: Vec<u32> = vec![1, 2, 3, 5, 7, 11, 13, 17];
    let remaining = (0..=9).collect();
    let res = check(String::new(), remaining, 0, &divisors);
    println!("Res: {res}");
}
