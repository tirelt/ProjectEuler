use std::fs;

fn alpha_value(s: &str) -> i32 {
    s.chars().map(|c| c as i32 - 'A' as i32 + 1).sum()
}
fn generate_triangle_numbers(numbers: &mut Vec<i32>, n: &mut i32, target: i32) {
    while *numbers.last().unwrap() < target {
        *n += 1;
        numbers.push(*n * (*n + 1) / 2);
    }
}
fn main() {
    let mut file = fs::read_to_string("words.txt").unwrap();
    file.retain(|c| c != '"');
    let mut triangle_numbers = vec![1];
    let mut n = 1;
    let mut res = 0;
    for w in file.split(',') {
        let score = alpha_value(w);
        generate_triangle_numbers(&mut triangle_numbers, &mut n, score);
        if triangle_numbers.contains(&score) {
            res += 1;
        }
    }

    println!("Res: {res}");
}
