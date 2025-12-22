use std::collections::BTreeMap;
fn compute_score(mut n: u64) -> u64 {
    let mut score = 0;
    while n > 0 {
        let digit = n % 10;
        score += (10 as u64).pow(digit as u32);
        n /= 10;
    }
    score
}
fn main() {
    let mut memo: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    let threshold = 5;
    let mut res = 1 << 63;
    for i in 1..10_000 as u64 {
        let cube = i.pow(3);
        if cube == 41063625 || cube == 56623104 || cube == 66430125 {
            println!("");
        }
        let score = compute_score(cube);
        let entry = memo.entry(score).or_insert(Vec::new());
        entry.push(cube);
    }
    // loop outside to make sure exactly 5 as per subject
    for (_, nums) in memo {
        if nums.len() == threshold {
            let smallest = nums.iter().min().unwrap();
            if res > *smallest {
                res = *smallest;
            }
        }
    }
    println!("Res: {res}");
}
