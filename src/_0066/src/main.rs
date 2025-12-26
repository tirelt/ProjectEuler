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

fn main() {
    let start = Instant::now();
    let mut res = 0;
    let mut max_x = 0;
    for d in 2..=1_000 as i128 {
        if d == 23 {
            println!("");
        }
        let (coeffs, cycle) = find_frac_representation(d as u64);
        let sqrt = (d as f64).sqrt() as i128;
        if sqrt.pow(2) == d {
            continue;
        }
        let mut x: Vec<i128> = vec![0, 1];
        let mut y: Vec<i128> = vec![1, 0];
        for a in coeffs {
            x.push(a as i128 * x[x.len() - 1] + x[x.len() - 2]);
            y.push(a as i128 * y[y.len() - 1] + y[y.len() - 2]);
            if x[x.len() - 1].pow(2) - d as i128 * y[y.len() - 1].pow(2) == 1 {
                break;
            }
        }
        let mut i = 0;
        while x[x.len() - 1].pow(2) - d as i128 * y[y.len() - 1].pow(2) != 1 {
            let a = cycle[i % cycle.len()];
            i += 1;
            x.push(a as i128 * x[x.len() - 1] + x[x.len() - 2]);
            y.push(a as i128 * y[y.len() - 1] + y[y.len() - 2]);
        }
        //println!("solved: {:?}", (d, x[x.len() - 1], y[y.len() - 1]));
        if x[x.len() - 1] > max_x {
            max_x = x[x.len() - 1];
            res = d;
        }
    }
    println!("Res: {:?}", res);
    println!("Duration: {}ms", start.elapsed().as_millis())
}
