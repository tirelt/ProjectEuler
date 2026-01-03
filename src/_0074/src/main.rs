use std::collections::HashSet;

fn calc_sum(mut n: i32, frac: &Vec<i32>) -> i32 {
    let mut res = 0;
    while n > 0 {
        res += frac[(n % 10) as usize];
        n /= 10;
    }
    res
}
fn main() {
    let mut frac = vec![1];
    let mut curr = 1;
    for i in 1..10 {
        curr *= i;
        frac.push(curr);
    }
    let mut res = 0;
    for i in 1..1_000_000 {
        println!("{i}");
        if i == 69 {
            println!("");
        }
        let mut memo = HashSet::new();
        let mut i = i;
        let mut counter = 0;
        while memo.insert(i) {
            i = calc_sum(i, &frac);
            counter += 1;
        }
        if counter == 60 {
            //exatly 60
            res += 1;
        }
    }
    println!("Res: {res}");
}
