use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn find_divisor(i: i32, memo: &mut BTreeMap<i32, BTreeSet<i32>>) {
    if let Some(_) = memo.get(&i) {
        return;
    } else {
        let mut res: BTreeSet<i32> = BTreeSet::new();
        for j in 1..i {
            if i % j == 0 {
                res.insert(j);
                find_divisor(j, memo);
                res = res.union(&memo[&j]).cloned().collect();
            }
        }
        memo.insert(i, res);
    }
}
fn main() {
    let limit = 28_123;
    let mut abundants = Vec::new();
    let mut memo: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    for i in 1..=limit {
        find_divisor(i, &mut memo);
        let sum: i32 = memo[&i].iter().sum();
        if sum > i {
            abundants.push(i);
        }
    }
    let mut numbers = BTreeMap::new();
    for i in 1..=limit {
        numbers.insert(i, true);
    }
    for i in 0..abundants.len() {
        for j in i..abundants.len() {
            let num = (abundants[i] + abundants[j]) as i32;
            if num <= limit {
                *numbers.get_mut(&num).unwrap() = false;
            }
        }
    }
    let mut res = 0;
    for (k, v) in numbers {
        if v {
            res += k;
        }
    }
    println!("Res: {res}");
}
