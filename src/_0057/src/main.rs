fn add_(mut a: Vec<u32>, mut b: Vec<u32>) -> Vec<u32> {
    if a.len() > b.len() {
        let temp = a;
        a = b;
        b = temp;
    }
    let mut keep = 0;
    for i in 0..a.len() {
        let temp = a[i] + b[i] + keep;
        keep = temp / 10;
        b[i] = temp % 10;
    }
    for i in a.len()..b.len() {
        let temp = b[i] + keep;
        keep = temp / 10;
        b[i] = temp % 10;
    }
    while keep > 0 {
        b.push(keep % 10);
        keep /= 10;
    }
    b
}
fn main() {
    let mut n: Vec<u32> = vec![3];
    let mut d: Vec<u32> = vec![2];
    let mut res = 0;
    for _ in 2..=1_000 {
        let temp = d.clone();
        d = add_(n, d);
        n = add_(d.clone(), temp);
        if n.len() > d.len() {
            res += 1;
        }
    }
    println!("Res: {res}");
}
