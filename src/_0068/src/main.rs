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
    let mut res_all = String::new();
    for i in starting_node..(starting_node + n) {
        let i = i % n;
        let last_index = if i < n - 1 { i + n + 1 } else { n };
        res_all.push_str(&nodes[i].unwrap().to_string());
        res_all.push_str(&nodes[i + n].unwrap().to_string());
        res_all.push_str(&nodes[last_index].unwrap().to_string());
    }
    return res_all.parse().unwrap();
}

fn brute_force(
    nodes: &mut Vec<Node>,
    i: usize,
    remaming: &mut Vec<bool>,
    target: i32,
    n: usize,
    res_all: &mut BTreeSet<i128>,
    n_digit: i32,
) {
    if i == nodes.len() {
        let solution = build_solution(nodes, n);
        if (solution as f64).log10() as i32 + 1 == 16 {
            //println!("{:?}", solution);
            if res_all.insert(solution) {
                //println!("{:?}", nodes);
            }
        }
    }
    for j in 0..remaming.len() {
        if !remaming[j] {
            continue;
        }
        remaming[j] = false;
        nodes[i] = Node::Some(1 + j as i32);
        if is_valid(i, n, target, nodes) {
            brute_force(nodes, i + 1, remaming, target, n, res_all, n_digit);
        }
        //backtracking
        remaming[j] = true;
        nodes[i] = Node::None;
    }
}
fn main() {
    let start = Instant::now();
    let n = 5;
    let n_digit = 16;
    let mut nodes = vec![Node::None; n * 2];
    let mut remaining = vec![true; 2 * n];
    let mut res_all = BTreeSet::new();
    for target in 6..27 {
        let prev_n_solutions = res_all.len();
        brute_force(
            &mut nodes,
            0,
            &mut remaining,
            target,
            n,
            &mut res_all,
            n_digit,
        );
        let n_solutions = res_all.len();
        {
            println!(
                "Target {target} admits {} solutions",
                n_solutions - prev_n_solutions
            )
        }
    }
    let res = res_all.last().unwrap();
    println!("res_all: {res}");
    println!("Duration: {}ms", start.elapsed().as_millis())
}
