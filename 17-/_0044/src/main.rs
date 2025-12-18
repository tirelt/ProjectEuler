fn generate_pentagonal_numbers(target: u64, n: &mut u64, numbers: &mut Vec<u64>) {
    while *numbers.last().unwrap() < target {
        *n += 1;
        numbers.push(*n * (*n * 3 - 1) / 2);
    }
}
struct Pair {
    l: u64,
    h: u64,
    d: u64,
}
impl Pair {
    fn new(l: u64, h: u64) -> Self {
        Pair { l, h, d: (h - l) }
    }
}
fn main() {
    let mut pentagonal_numbers = vec![1];
    let mut n = 1;
    generate_pentagonal_numbers(10_000_000, &mut n, &mut pentagonal_numbers);
    let mut pairs = Vec::new();
    for i in 0..pentagonal_numbers.len() {
        for j in (1 + i)..pentagonal_numbers.len() {
            pairs.push(Pair::new(pentagonal_numbers[i], pentagonal_numbers[j]));
        }
    }
    let mut res = 0;
    pairs.sort_by(|a, b| a.d.cmp(&b.d));
    for Pair { l, h, d } in pairs.iter() {
        if pentagonal_numbers.contains(&(l + h)) && pentagonal_numbers.contains(&(h - l)) {
            res = *d;
            println!("l: {l}, h: {h}");
            break;
        }
    }
    println!("Res: {res}");
}
