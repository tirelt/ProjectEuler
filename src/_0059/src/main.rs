use std::fs;

fn decypher(text: &Vec<u8>, key: &Vec<u8>) -> String {
    let mut text_decyphered = Vec::new();
    let mut i = 0;
    for t in text {
        text_decyphered.push((t ^ key[i]) as char);
        i = (i + 1) % key.len();
    }
    let mut res = String::new();
    for t in text_decyphered {
        res.push(t);
    }
    res
}
fn main() {
    let file = fs::read_to_string("text.txt").unwrap();
    let mut text: Vec<u8> = Vec::new();
    for line in file.split(',') {
        text.push(line.parse().unwrap());
    }
    let mut res: u64 = 0;
    for k1 in 'a'..='z' {
        for k2 in 'a'..='z' {
            for k3 in 'a'..='z' {
                let key: Vec<u8> = vec![k1 as u8, k2 as u8, k3 as u8];
                let text_decyphered = decypher(&text, &key);
                let mut words = Vec::new();
                let mut max_letter = 0;
                for word in text_decyphered.split(' ') {
                    words.push(word);
                    max_letter = std::cmp::max(max_letter, word.len());
                }
                if max_letter < 22 && words.len() > text_decyphered.len() / 20 {
                    res = text_decyphered.chars().map(|x| x as u64).sum();
                    break;
                }
            }
        }
    }
    println!("Res: {res}");
}
