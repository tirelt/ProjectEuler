use std::collections::BTreeMap;

fn get_fact(n: u64, table: &mut Vec<u64>) {
    match n {
        0 => table.push(1),
        k => {
            get_fact(k - 1, table);
            table.push(table.last().unwrap() * k);
        }
    }
}
fn find_number(n: u64, numbers: &mut BTreeMap<u64, bool>) -> Option<u64> {
    let mut counter = 0;
    for (num, available) in numbers {
        if *available {
            if counter == n {
                *available = false;
                return Some(*num);
            }
            counter += 1;
        }
    }
    return None;
}
fn first_number(numbers: &mut BTreeMap<u64, bool>) -> Option<u64> {
    for (num, available) in numbers.iter_mut() {
        if *available {
            *available = false;
            return Some(*num);
        }
    }
    None
}

fn main() {
    let mut target: u64 = 1_000_000 - 1;
    let n_digit = 9;
    let mut table = Vec::new();
    get_fact(n_digit, &mut table);
    let mut numbers = BTreeMap::new();
    for i in 0..=n_digit {
        numbers.insert(i, true);
    }
    let mut res = 0;
    let mut num;
    for i in (0..=n_digit).rev() {
        let i = i as usize;
        if target > 0 {
            let first = target / &table[i];
            num = find_number(first, &mut numbers).unwrap();
            target -= first * &table[i];
        } else {
            num = first_number(&mut numbers).unwrap();
        }
        res += num * (10 as u64).pow(i as u32);
    }
    println!("Res: {res}");
}
