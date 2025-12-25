use pe_lib::big_numbers::BigNum;
use std::time::Instant;
/*
fn exp_frac_representation(n: usize) -> Vec<i64> {
    let mut d_exp: i64 = 0;
    let mut d_c: i64 = 1;
    let mut n_exp: i64 = 1;
    let mut n_c: i64 = 0;
    let exp = (1 as f64).exp();
    let mut coeffs = Vec::new();
    while coeffs.len() < n {
        let floor = ((n_c as f64 + n_exp as f64 * exp) / (d_c as f64 + d_exp as f64 * exp)) as i64;
        let (d_exp_temp, d_c_temp) = (d_exp, d_c);
        d_exp = n_exp - floor * d_exp;
        d_c = n_c - floor * d_c;
        (n_c, n_exp) = (d_c_temp, d_exp_temp);
        coeffs.push(floor);
    }
    coeffs
}
*/
fn construct_frac(representation: Vec<i64>) -> (BigNum, BigNum) {
    let mut num = BigNum::new_from_u64(1);
    let mut den = BigNum::new_from_u64(representation[0] as u64);
    for i in 1..representation.len() {
        let new_num = den.clone();
        den = &(representation[i] as u64 * &den) + &num;
        num = new_num;
    }
    (num, den)
}
fn main() {
    let start = Instant::now();
    //let representation = exp_frac_representation(20);
    // the representation looks very predictable (2,1,2,1,1,4,1,1,6,1,1,8,1,1,10,11,12,...) but we cannot compute more term due to floating point precision
    let mut next_last = 4;
    let mut representation = vec![2, 1, 2];
    for i in 3..100 {
        if i % 3 == 2 {
            representation.push(next_last);
            next_last += 2;
        } else {
            representation.push(1);
        }
    }
    let (_num, den) = construct_frac(representation.into_iter().rev().collect());
    let res: u64 = den.digits.iter().sum();
    println!("Res: {res}");
    println!("Duration: {}micros", start.elapsed().as_nanos() / 1_000)
}
