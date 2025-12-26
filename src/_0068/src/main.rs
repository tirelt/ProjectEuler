use std::collections::BTreeSet;
use std::time::Instant;

#[derive(Clone, Debug)]
enum Node {
    Some(i32),
    None,
}

impl Node {
    fn unwrap(&self) -> i32 {
        match self {
            Node::Some(x) => x.clone(),
            _ => panic!("unwrapped empty Node"),
        }
    }
}

fn is_valid(i: usize, n: usize, target: i32, nodes: &Vec<Node>) -> bool {
    if i < n {
        let last_index = if i < n - 1 { i + n + 1 } else { n };
        if let (Node::Some(a), Node::Some(b), Node::Some(c)) =
            (&nodes[i], &nodes[i + n], &nodes[last_index])
        {
            if a + b + c != target {
                return false;
            }
        }
        return true;
    } else {
        let second_origin = if i == n { n - 1 } else { i - n - 1 };
        return is_valid(i - n, n, target, nodes) && is_valid(second_origin, n, target, nodes);
    }
}

fn build_solution(nodes: &Vec<Node>, n: usize) -> i128 {
    let mut starting_node = 0;
    let mut current_min = nodes.first().unwrap().unwrap();
    for i in 1..n {
        let current = nodes[i].unwrap();
        if current_min > current {
            current_min = current;
            starting_node = i;
        }
    }
    let mut res: i128 = 0;
    for i in starting_node..(starting_node + n) {
        let i = i % n;
        let last_index = if i < n - 1 { i + n + 1 } else { n };
        res = res * 1_000
            + (nodes[i].unwrap() * 100 + nodes[i + n].unwrap() * 10 + nodes[last_index].unwrap())
                as i128;
    }
    return res;
}

fn brute_force(
    nodes: &mut Vec<Node>,
    i: usize,
    remaming: &mut Vec<bool>,
    target: i32,
    n: usize,
    res: &mut BTreeSet<i128>,
) {
    if i == nodes.len() {
        let solution = build_solution(nodes, n);
        //println!("{:?}", nodes);
        //println!("{:?}", solution);
        res.insert(solution);
    }
    for j in 0..remaming.len() {
        if !remaming[j] {
            continue;
        }
        remaming[j] = false;
        nodes[i] = Node::Some(1 + j as i32);
        if is_valid(i, n, target, nodes) {
            brute_force(nodes, i + 1, remaming, target, n, res);
        }
        //backtracking
        remaming[j] = true;
        nodes[i] = Node::None;
    }
}
fn main() {
    let start = Instant::now();
    let n = 5;
    let target = 16;
    let mut nodes = vec![Node::None; n * 2];
    let mut remaining = vec![true; 2 * n];
    let mut res = BTreeSet::new();
    brute_force(&mut nodes, 0, &mut remaining, target, n, &mut res);
    let res = res.last().unwrap();
    println!("Res: {res}");
    println!("Duration: {}ms", start.elapsed().as_millis())
}
