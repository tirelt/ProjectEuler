fn find_nth_digit(n: u64) -> u32 {
    let mut n = n - 1;
    let mut i = 0;
    let mut mult = 9;
    while n > (i + 1) * mult {
        n = n - (i + 1) * mult;
        mult *= 10;
        i += 1;
    }
    let lb = (10 as u64).pow(i as u32);
    let q = n / (i + 1);
    let r = (n % (i + 1)) as u32;
    let number = lb + q;
    let n_digit = (number as f64).log10() as u32;
    let exp = n_digit - r;
    let res = number as u32 / (10 as u32).pow(exp) % 10;
    res
}
fn main() {
    let res = find_nth_digit(1)
        * find_nth_digit(10)
        * find_nth_digit(100)
        * find_nth_digit(1_000)
        * find_nth_digit(10_000)
        * find_nth_digit(100_000)
        * find_nth_digit(1_000_000);
    println!("Res: {res}");
}
