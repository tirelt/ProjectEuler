fn test_n(n: u64, exp: u32) -> u64 {
    let q = n / 10;
    match q {
        0 => n.pow(exp),
        _ => (n % 10).pow(exp) + test_n(q, exp),
    }
}
fn main() {
    let exp = 5;
    let mut res = 0;
    for i in 2..1_000_000 {
        let v = test_n(i, exp);
        if i == v {
            res += i;
        }
    }
    println!("Res {res}");
}
