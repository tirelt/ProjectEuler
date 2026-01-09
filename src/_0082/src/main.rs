use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Eq, PartialEq)]
struct ItemWithWeight<T> {
    item: T,
    weight: u64,
}

impl<T: Eq> Ord for ItemWithWeight<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T: Eq> PartialOrd for ItemWithWeight<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
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
    let mut queue = BinaryHeap::new();
    for i in 0..matrix.len() {
        queue.push(ItemWithWeight {
            weight: matrix[i][0],
            item: (i, 0),
        });
        memo.insert((i, 0), matrix[i][0]);
    }
    while let Some(ItemWithWeight { item: p, weight: w }) = queue.pop() {
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
        for n_p in next_p {
            let n_w = w + matrix[n_p.0][n_p.1];
            if let Some(&o_w) = memo.get(&n_p) {
                if o_w < n_w {
                    continue;
                }
            }
            queue.push(ItemWithWeight {
                weight: n_w,
                item: n_p,
            });
            memo.insert(n_p, n_w);
        }
    }
    let mut res = memo[&(0, matrix[0].len() - 1)];
    for i in 1..matrix.len() {
        res = std::cmp::min(res, memo[&(i, matrix[0].len() - 1)])
    }
    println!("Res: {}", res);
    println!("Duration: {}ms", start.elapsed().as_millis());
}
