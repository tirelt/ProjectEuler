use std::collections::BTreeSet;

fn get_fact(n: u64, table: &mut Vec<u64>) {
    match n {
        0 => table.push(1),
        k => {
            get_fact(k - 1, table);
            table.push(table.last().unwrap() * k);
        }
    }
}
fn main() {
    let target: u64 = 1_000_000;
    let mut table = Vec::new();
    get_fact(10, &mut table);
    let mut numbers = BTreeSet::new();
    for i in 0..9 {
        numbers.insert(i);
    }
    let mut num = 0;
    let mut counter = 10;
    let mut curr_number = 0;
    let mut res = 0;
    for i in (0..=9).rev() {
        let first = target / &table[i];
        curr_number += first * &table[i];
        res += first * (10 as u32).pow(i);
    }
}
