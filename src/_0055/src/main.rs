fn is_palindrome(num: &Vec<i32>) -> bool {
    let rev: Vec<&i32> = num.iter().rev().collect();
    num.iter()
        .zip(rev)
        .fold(true, |acc, (a, b)| acc && (a == b))
}
fn next(mut num: Vec<i32>) -> Vec<i32> {
    let rev: Vec<i32> = num.iter().copied().rev().collect();
    let mut keep = 0;
    for i in 0..num.len() {
        let new = num[i].clone() + rev[i] + keep;
        keep = new / 10;
        num[i] = new % 10;
    }
    if keep > 0 {
        num.push(keep);
    }
    num
}
fn main() {
    let mut res = 0;
    'outer: for i in 1..10_000 {
        let mut num = Vec::new();
        let mut i_ = i;
        while i_ > 0 {
            num.push(i_ % 10);
            i_ /= 10;
        }
        for _ in 0..50 {
            num = next(num);
            if is_palindrome(&num) {
                continue 'outer;
            }
        }
        res += 1;
    }
    println!("Res: {res}");
}
