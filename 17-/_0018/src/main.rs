use std::fs;

fn main() {
    let file = fs::read_to_string("input").expect("should read");
    let mut table: Vec<Vec<u32>> = Vec::new();
    for ligne in file.lines() {
        let mut vec_ligne: Vec<u32> = Vec::new();
        for words in ligne.split_whitespace() {
            vec_ligne.push(words.trim().parse().unwrap());
        }
        table.push(vec_ligne);
    }
    println!("{table:?}");
}
