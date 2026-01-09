use pe_lib::big_numbers::BigNum;
use std::collections::HashMap;

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
    let mut num = BigNum::new_from_u64(coeffs[0] as u64);
    let mut prev_num = BigNum::new_from_u64(1 as u64);
    let mut den = BigNum::new_from_u64(1 as u64);
    let mut prev_den = BigNum::new_from_u64(0 as u64);
    for i in 1..coeffs.len() {
        let new_num = &(&num * coeffs[i] as u64) + &prev_num;
        let new_den = &(&den * coeffs[i] as u64) + &prev_den;
        (prev_num, prev_den) = (num, den);
        (num, den) = (new_num, new_den);
    }
    let mut i = 0;
    while den.len() < n_digits {
        let new_num = &(&num * cycle[i] as u64) + &prev_num;
        let new_den = &(&den * cycle[i] as u64) + &prev_den;
        (prev_num, prev_den) = (num, den);
        (num, den) = (new_num, new_den);
        i = (i + 1) % cycle.len();
    }
    (num, den)
}
fn main() {
    let n_digits = 100;
    let mut quotient;
    let mut rest;
    let mut res_total = 0;
    let mut digits = Vec::new();
    for n in 1..=100 {
        let (coeffs, cycle) = find_frac_representation(n);
        digits.clear();
        if cycle.len() > 0 {
            let mut res = 0;
            let (num, den) = construct_frac(n_digits * 2, coeffs, cycle);
            (quotient, rest) = num.div(&den);
            for v in quotient.digits.iter() {
                digits.push(*v);
            }
            while digits.len() < n_digits {
                rest.mult_by_10_pow(1);
                (quotient, rest) = rest.div(&den);
                for v in quotient.digits.iter() {
                    res += v;
                    digits.push(*v);
                    if digits.len() >= n_digits {
                        continue;
                    }
                }
                if quotient.len() == 0 {
                    digits.push(0);
                }
            }
            let res: u64 = digits.iter().sum();
            res_total += res;
            //println!("{} {}", n, res);
        }
    }
    println!("Res: {}", res_total);
}
