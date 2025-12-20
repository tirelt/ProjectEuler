fn gcd(mut a: u128, mut b: u128) -> u128 {
    if a == b {
        return a;
    }
    if b < a {
        let temp = a;
        a = b;
        b = temp;
    }
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}
fn n_digit(n: u128) -> u32 {
    (n as f64).log10() as u32
}
fn main() {
    let mut n: u128 = 3;
    let mut d: u128 = 2;
    let mut res = 0;
    for k in 2..=1_000 {
        n = n + d;
        let temp = d;
        d = n;
        n = temp + d;
        let g = gcd(n, d);
        if g > 1 {
            println!("Hello, world!");
        }
        n = n / g;
        d = d / g;
        if n_digit(n) > n_digit(d) {
            res += 1;
        }
    }
    println!("Res: {res}");
}
