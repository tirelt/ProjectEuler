use pe_lib::big_numbers::BigNum;
use std::collections::HashMap;
use std::time::Instant;

fn find_frac_representation(n: u64) -> (Vec<u64>, Vec<u64>) {
    let mut memo: HashMap<(u64, u64), u64> = HashMap::new();
    let mut sqrt = (n as f64).sqrt() as u64;
    let mut dem: u64 = 1;
    let mut num_sub = sqrt;
    let mut coeffs = vec![sqrt];
    if sqrt.pow(2) == n {
        return (coeffs, Vec::new());
    }
    let mut counter = 1;
    while memo.get(&(num_sub, dem)).is_none() {
        memo.insert((num_sub, dem).clone(), counter);
        let new_dem = n - num_sub.pow(2);
        counter += 1;
        if new_dem % dem != 0 {
            panic!("Can get here ?!");
        }
        dem = new_dem / dem;
        sqrt = (((n as f64).sqrt() + num_sub as f64) / dem as f64) as u64;
        num_sub = dem * sqrt - num_sub;
        coeffs.push(sqrt);
    }
    let start_cycle_index = memo.get(&(num_sub, dem)).unwrap().clone() as usize;
    let cycle: Vec<u64> = coeffs.iter().skip(start_cycle_index).copied().collect();
    (coeffs.into_iter().take(start_cycle_index).collect(), cycle)
}

fn construct_frac(n_digits: usize, coeffs: Vec<u64>, cycle: Vec<u64>) -> (BigNum, BigNum) {
    let mut num = BigNum::new_from_u64(1);
    let mut den = BigNum::new_from_u64(coeffs[0] as u64);
    for i in 1..coeffs.len() {
        let new_num = den.clone();
        den = &(coeffs[i] as u64 * &den) + &num;
        num = new_num;
    }
    let mut i = 0;
    while den.len() < n_digits {
        let new_num = den.clone();
        den = &(cycle[i] as u64 * &den) + &num;
        num = new_num;
        i = (i + 1) % cycle.len();
    }
    (num, den)
}
fn main() {
    let n = 2;
    for n in 2..100 {
        let (coeffs, cycle) = find_frac_representation(n);
        if cycle.len() > 0 {
            let (num, den) = construct_frac(100, coeffs, cycle);
            let test = num < den;
        }
    }
    println!("Hello, world!");
}
