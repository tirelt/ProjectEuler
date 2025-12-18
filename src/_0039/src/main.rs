use std::collections::BTreeMap;
fn main() {
    let mut memo = BTreeMap::new();
    let max_p: i32 = 1000;
    for i in 1..=1000 {
        for j in i..=1000 {
            let k = (i as i32).pow(2) + (j as i32).pow(2);
            let sqrt_k = (k as f64).sqrt() as i32;
            let p = i + j + sqrt_k;
            if p <= max_p && k == sqrt_k * sqrt_k {
                *memo.entry(p).or_insert(0) += 1;
            }
        }
    }
    let mut res = 0;
    let mut curr_max = 0;
    for (p, v) in memo {
        if v > curr_max {
            curr_max = v;
            res = p;
        }
    }
    println!("Res: {res} - nbre: {curr_max}");
}
