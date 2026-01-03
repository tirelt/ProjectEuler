use std::collections::BTreeMap;
fn main() {
    let mut squares = vec![0];
    let max_i = (1_500_000.0 / (2.0 + (2.0 as f64).sqrt())) as u64;
    for i in 1..max_i as u64 {
        let s = i.pow(2);
        squares.push(s);
    }
    println!("Max i {max_i}");
    let mut memo = BTreeMap::new();
    'loop_i: for i in 1..squares.len() {
        if i % 1000 == 0 {
            println!("{i}");
        }
        for j in (i + 1)..squares.len() {
            let r = squares[i] + squares[j];
            let sqrt = (r as f64).sqrt() as u64;
            let l = i as u64 + j as u64 + sqrt;
            if l > 1_500_000 {
                continue 'loop_i;
            }
            if sqrt.pow(2) == r {
                *memo.entry(l).or_insert(0) += 1;
            }
        }
    }
    println!("Res: {}", memo[&120]);
}
