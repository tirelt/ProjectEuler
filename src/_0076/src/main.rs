fn rec(rest: u64, numbers: &mut Vec<u64>, res: &mut u64) {
    if rest == 0 {
        *res += 1;
        return;
    }
    let mut max_i = if numbers.len() > 0 {
        numbers.last().unwrap().clone()
    } else {
        rest.clone()
    };
    max_i = std::cmp::min(max_i, rest);
    for i in (1..=max_i).rev() {
        numbers.push(i);
        rec(rest - i, numbers, res);
        // backtrack
        numbers.pop();
    }
}
fn main() {
    let n = 100;
    let mut res = 0;
    let mut numbers = Vec::new();
    rec(n, &mut numbers, &mut res);
    println!("Res: {}", res - 1);
}
