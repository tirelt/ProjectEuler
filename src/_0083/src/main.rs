use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("matrix.txt").unwrap();
    let mut matrix = Vec::new();
    for line in input.lines() {
        let mut matrix_line = Vec::new();
        for n in line.split(',') {
            let n: u64 = n.parse().unwrap();
            matrix_line.push(n);
        }
        matrix.push(matrix_line);
    }
    let mut memo = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<u64>, (usize, usize))> = BinaryHeap::new();
    for i in 0..=0 {
        queue.push((Reverse(matrix[i][0]), (i, 0)));
        memo.insert((i, 0), matrix[i][0]);
    }
    while let Some((Reverse(w), p)) = queue.pop() {
        if let Some(&o_w) = memo.get(&p) {
            if o_w < w {
                continue;
            }
        }
        let mut next_p = Vec::new();
        if p.0 < matrix.len() - 1 {
            next_p.push((p.0 + 1, p.1));
        }
        if p.0 != 0 {
            next_p.push((p.0 - 1, p.1));
        }
        if p.1 < matrix[0].len() - 1 {
            next_p.push((p.0, p.1 + 1));
        }
        if p.1 != 0 {
            next_p.push((p.0, p.1 - 1));
        }
        for n_p in next_p {
            let n_w = w + matrix[n_p.0][n_p.1];
            if let Some(&o_w) = memo.get(&n_p) {
                if o_w < n_w {
                    continue;
                }
            }
            queue.push((Reverse(n_w), n_p));
            memo.insert(n_p, n_w);
        }
    }
    let res = memo[&(matrix[0].len() - 1, matrix[0].len() - 1)];
    println!("Res: {}", res);
    println!("Duration: {}ms", start.elapsed().as_millis());
}
