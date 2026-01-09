use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
fn main() {
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
    memo.insert((0, 0), matrix[0][0]);
    let mut queue = BTreeMap::new();
    queue.insert(matrix[0][0], (0, 0));
    while queue.len() > 0 {
        let (w, p) = queue.pop_first().unwrap();
        if let Some(&o_w) = memo.get(&p) {
            if o_w < w {
                continue;
            }
        }
        let mut next_p = Vec::new();
        if p.0 < matrix.len() - 1 {
            next_p.push((p.0 + 1, p.1));
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
            queue.insert(n_w, n_p);
            memo.insert(n_p, n_w);
        }
    }

    println!("Res: {}", memo[&(matrix.len() - 1, matrix[0].len() - 1)]);
}
