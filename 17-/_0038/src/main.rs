use std::collections::HashSet;
use std::ops::Range;

fn test_range(res: &mut u64, range: Range<u64>, mult: Vec<u64>) {
    'outer: for i in range {
        let mut n_str = String::new();
        for m in mult.iter() {
            n_str.push_str(&(m * i).to_string());
        }
        let mut memo = HashSet::new();
        for c in n_str.chars() {
            if !memo.insert(c) {
                continue 'outer;
            }
        }
        if n_str.len() != 9 || memo.get(&'0').is_some() {
            continue;
        }
        let n: u64 = n_str.parse().unwrap();
        if n > *res {
            *res = n;
        }
    }
}
fn main() {
    let mut res = 0;
    test_range(&mut res, 5_000..10_000, (1..=2).collect());
    test_range(&mut res, 100..334, (1..=3).collect());
    test_range(&mut res, 25..34, (1..=4).collect());
    test_range(&mut res, 5..20, (1..=5).collect());
    test_range(&mut res, 3..4, (1..=6).collect());
    test_range(&mut res, 1..2, (1..=9).collect());
    println!("Res: {res}");
}
