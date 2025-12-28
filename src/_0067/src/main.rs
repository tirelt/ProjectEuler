use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let file = fs::read_to_string("triangle.txt").expect("should read");
    let mut table: Vec<Vec<u32>> = Vec::new();
    for ligne in file.lines() {
        let mut vec_ligne: Vec<u32> = Vec::new();
        for words in ligne.split_whitespace() {
            vec_ligne.push(words.trim().parse().unwrap());
        }
        table.push(vec_ligne);
    }
    let mut memo: HashMap<(usize, usize), u32> = HashMap::from([((0, 0), table[0][0])]);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);
    let mut max = 0;
    let i_max = table.len() - 1;
    while queue.len() > 0 {
        let front = queue.pop_front().unwrap();
        let value = memo.get(&front).unwrap().clone();
        if front.0 + 1 < table.len() {
            let new_pos_vec = vec![(front.0 + 1, front.1), (front.0 + 1, front.1 + 1)];
            for new_pos in new_pos_vec {
                let new_value = table[new_pos.0][new_pos.1] + value;
                let old_value_ref = memo.entry(new_pos).or_insert(new_value.clone());
                if *old_value_ref <= new_value {
                    *old_value_ref = new_value;
                    queue.push_back(new_pos);
                    if new_pos.0 == i_max && max < new_value {
                        max = new_value;
                    }
                }
            }
        }
    }
    println!("Max: {max}");
}
