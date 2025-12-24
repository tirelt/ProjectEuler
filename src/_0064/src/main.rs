fn find_frac_representation(n: u64) {
    let sqrt = (n as f64).sqrt() as u64;
    let mut coeffs = vec![sqrt];
    let mut dem: u64 = 1;
    let mut num_sub = sqrt;

    for i in 0..10 {
        let mut new_dem = n - num_sub.pow(2);
        if new_dem % dem != 0 {
            println!("");
        }
        new_dem /= dem;
    }
    coeffs.push()
}
fn main() {
    println!("Hello, world!");
}
