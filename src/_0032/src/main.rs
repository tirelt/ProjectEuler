use std::collections::BTreeSet;
use std::ops::Range;

fn is_pandigital(product: Vec<i32>) -> bool {
    let mut seen = BTreeSet::new();
    for n in product {
        let mut n = n;
        while n > 0 {
            if n % 10 == 0 || !seen.insert(n % 10) {
                return false;
            }
            n = n / 10;
        }
    }
    true
}

fn check_range(
    range_i: Range<i32>,
    range_j: Range<i32>,
    low_k: i32,
    high_k: i32,
    all_res: &mut BTreeSet<i32>,
    res: &mut i32,
) {
    for i in range_i {
        for j in range_j.clone() {
            let k = i * j;
            if k > low_k && k < high_k {
                if i == 12 && j == 114 {
                    println!("test");
                }
                if is_pandigital(vec![i, j, k]) {
                    if all_res.insert(k) {
                        *res += k;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut all_res: BTreeSet<i32> = BTreeSet::new();
    let mut res = 0;
    check_range(1..10, 1_000..9_999, 1_000, 9_999, &mut all_res, &mut res);
    check_range(10..99, 100..999, 1_000, 9_999, &mut all_res, &mut res);
    println!("Res: {res}");
}
