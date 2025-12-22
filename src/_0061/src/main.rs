use pe_lib::combinatorics::generate_permutations;
use std::{collections::BTreeMap, time::Instant};

fn generate_triangle(n: u64) -> u64 {
    n * (n + 1) / 2
}
fn generate_square(n: u64) -> u64 {
    n * n
}
fn generate_pentagonal(n: u64) -> u64 {
    n * (n * 3 - 1) / 2
}
fn generate_hexagonal(n: u64) -> u64 {
    n * (n * 2 - 1)
}
fn generate_heptagonal(n: u64) -> u64 {
    n * (5 * n - 3) / 2
}
fn generate_octagonal(n: u64) -> u64 {
    n * (3 * n - 2)
}
fn check(
    permutation: &Vec<u64>,
    i: usize,
    curr: &mut Vec<u64>,
    values_map: &Vec<BTreeMap<u64, Vec<u64>>>,
    all_values: &Vec<Vec<u64>>,
    res: &mut Vec<Vec<u64>>,
) {
    if i == permutation.len() {
        let first = curr.first().unwrap();
        let last = curr.last().unwrap();
        if last % 100 == first / 100 {
            res.push(curr.clone());
        }
        return;
    }
    let mut nums = &all_values[permutation[i] as usize];
    if i > 0 {
        let last = curr.last().unwrap();
        let Some(l) = values_map[permutation[i] as usize].get(&(last % 100)) else {
            return;
        };
        nums = l;
    }
    for n in nums {
        curr.push(*n);
        check(permutation, i + 1, curr, values_map, all_values, res);
        curr.pop();
    }
}
fn main() {
    let now = Instant::now();

    let functions = vec![
        generate_triangle,
        generate_square,
        generate_pentagonal,
        generate_hexagonal,
        generate_heptagonal,
        generate_octagonal,
    ];

    //    let functions = vec![generate_triangle, generate_square, generate_pentagonal];
    let permuatations = generate_permutations(functions.len());
    let mut values_map = Vec::new();
    let mut all_values = Vec::new();
    for f in functions {
        let mut map = BTreeMap::new();
        let mut values = Vec::new();
        let mut n = 1;
        let mut v = f(1);
        while v < 10_000 {
            n += 1;
            v = f(n);
            if v > 999 && v < 10_000 {
                map.entry(v / 100).or_insert(Vec::new()).push(v);
                values.push(v);
            }
        }
        values_map.push(map);
        all_values.push(values);
    }
    let mut res = Vec::new();
    let mut curr = Vec::new();
    for permutation in permuatations {
        check(
            &permutation,
            0,
            &mut curr,
            &values_map,
            &all_values,
            &mut res,
        );
    }
    let sum: u64 = res[0].iter().sum();
    println!("Res: {sum}\n{:?}", res[0]);
    println!("{}ms", now.elapsed().as_millis());
}
