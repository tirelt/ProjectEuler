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
    let n = 10_000;
    let mut memo: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    let mut res = 0;
    for i in 1..=n {
        find_divisor(i, &mut memo);
        let sum: i32 = memo[&i].iter().sum();
        find_divisor(sum, &mut memo);
        let sum_: i32 = memo[&sum].iter().sum();
        if sum_ == i && i != sum {
            res += i;
        }
    }
    println!("Res: {res}");
}
