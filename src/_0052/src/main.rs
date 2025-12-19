fn get_n_digit(n: u64) -> u64 {
    (n as f64).log10() as u64 + 1
}
fn get_digits(mut n: u64) -> Vec<u64> {
    let mut res = vec![0; 10];
    while n > 0 {
        res[(n % 10) as usize] += 1;
        n /= 10;
    }
    res
}
fn main() {
    let mult = 6;
    let mut res = 0;
    'outer: for i in 1..10_000_000 {
        let i = i as u64;
        if get_n_digit(i) == get_n_digit(mult * i) {
            let digits = get_digits(i);
            for k in 2..=mult {
                if digits != get_digits(k * i) {
                    continue 'outer;
                }
            }
            res = i;
            break;
        }
    }
    println!("Res: {res}");
}
