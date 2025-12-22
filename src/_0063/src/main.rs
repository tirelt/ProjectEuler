use std::collections::BTreeSet;
use std::time::Instant;
fn n_digit(a: u128) -> u32 {
    (a as f64).log10() as u32 + 1
}
fn main() {
    /* d = round_up( n ln(a)/ln(10) )
    d = 1 for n = 0; so n: d(n) starts above n: n, so they can meet only if a < 10
     */
    let start = Instant::now();
    let mut memo = BTreeSet::new();
    memo.insert(1);

    for a in 2..10 as u128 {
        let mut pow: u128 = 1;
        for n in 1..100 as u32 {
            pow *= a;
            let d = n_digit(pow);
            if n == d {
                memo.insert(pow);
            }
            if n > d {
                break;
            }
        }
    }
    println!("Res: {}", memo.len());
    println!("Duration: {}micros", start.elapsed().as_nanos() / 1_000)
}
