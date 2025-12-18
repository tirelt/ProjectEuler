fn main() {
    let divisor: u64 = 10_000_000_000;
    let mut res = 0;
    for i in 1..=1000 {
        let mut temp = i;
        for _ in 1..i {
            temp = (temp * i) % divisor;
        }
        res = (res + temp) % divisor;
    }
    println!("Res: {res}");
}
