use std::fs;

struct Hand {
    cards:
}
fn main() {
    let file = fs::read_to_string("poker.txt").unwrap();
    for line in file.lines(){
        let counter = 0;
        for card in line.split(' ') {

        }
    }
    println!("Hello, world!");
}
