fn power(a: u32, b: u32) -> Vec<u32> {
    if b == 0 {
        return vec![1];
    }
    let mut res = Vec::new();
    let mut a_ = a;
    while a_ > 0 {
        res.push(a_ % 10);
        a_ /= 10;
    }
    let mut keep = 0;
    for _ in 1..b {
        for v in res.iter_mut() {
            let new_v = *v * a + keep;
            keep = new_v / 10;
            *v = new_v % 10;
        }
        while keep > 0 {
            res.push(keep % 10);
            keep = keep / 10;
        }
    }
    res
}
fn main() {
    let mut res = 0;
    for a in 2..100 {
        for b in 1..100 {
            // should optimize to not recompute the full power
            let r = power(a, b);
            let new: u32 = r.iter().sum();
            if new > res {
                res = new;
            }
        }
    }
    println!("Hello, world!");
}
