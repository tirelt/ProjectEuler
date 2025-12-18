use std::fs;

fn alpha_value(s: &str) -> i64 {
    s.chars().map(|c| c as i64 - 'A' as i64 + 1).sum()
}
fn main() {
    let file = fs::read_to_string("names.txt").unwrap();
    let mut names = Vec::new();
    for name in file.split(',') {
        let mut name = name.to_string();
        name.retain(|c| c != '"');
        names.push(name);
    }
    names.sort();
    let res: i64 = (0..names.len())
        .map(|i| alpha_value(&names[i]) * (i + 1) as i64)
        .sum();
    println!("Res: {res}");
}
