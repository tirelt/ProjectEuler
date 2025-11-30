fn main() {
    let n = 100;
    let number_digit: f64 = (1..=n).map(|x| (x as f64).ln() / (10 as f64).ln()).sum();
    println!("Approx numver of digit of 100! {number_digit}");
    let mut digit = vec![1];
    for i in 1..=n {
        let mut remainder = 0;
        for v in digit.iter_mut() {
            remainder += *v * i;
            *v = remainder % 10;
            remainder = remainder / 10;
        }
        while remainder > 0 {
            digit.push(remainder % 10);
            remainder = remainder / 10;
        }
    }
    let res: u32 = digit.iter().sum();
    println!("{res}");
}
