use std::collections::HashMap;
use std::time::Instant;

fn bfs(
    value: i32,
    coin_index: usize,
    coins: &Vec<i32>,
    memo: &mut HashMap<(i32, usize), u128>,
) -> u128 {
    if value == 0 {
        return 1;
    }
    if value < 0 {
        return 0;
    }
    if let Some(n) = memo.get(&(value, coin_index)) {
        return *n;
    }
    let mut res = 0;
    for i in coin_index..coins.len() {
        res += bfs(value - coins[i], i, coins, memo);
    }
    memo.insert((value, coin_index), res);
    res
}
fn main() {
    let coins = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let mut memo: HashMap<(i32, usize), u128> = HashMap::new();
    let start = Instant::now();
    let res = bfs(200, 0, &coins, &mut memo);
    let duration = start.elapsed();
    println!(
        "Res: {res} - {:.0} μs",
        duration.as_secs_f64() * 1_000_000.0
    );
}
