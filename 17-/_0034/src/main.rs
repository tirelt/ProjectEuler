fn fact(n: i32) -> i32 {
    match n {
        0 => 1,
        k => k * fact(k - 1),
    }
}
fn main() {
    let mut res = 0;
    for i in 10..1_000_000 {
        let mut i_ = i;
        let mut temp = 0;
        while i_ > 0 {
            temp += fact(i_ % 10);
            i_ = i_ / 10;
        }
        if temp == i {
            res += i;
        }
    }
    println!("Res: {res}");
}
