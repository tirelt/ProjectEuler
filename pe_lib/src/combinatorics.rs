pub fn generate_permutations(n: usize) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    let mut curr = Vec::with_capacity(n);
    let mut used = vec![false; n];

    backtrack(n, &mut curr, &mut used, &mut result);
    result
}

fn backtrack(n: usize, curr: &mut Vec<u64>, used: &mut Vec<bool>, result: &mut Vec<Vec<u64>>) {
    if curr.len() == n {
        result.push(curr.clone());
        return;
    }

    for i in 0..n as u64 {
        if !used[i as usize] {
            used[i as usize] = true;
            curr.push(i);
            backtrack(n, curr, used, result);

            // Backtrack
            curr.pop();
            used[i as usize] = false;
        }
    }
}
