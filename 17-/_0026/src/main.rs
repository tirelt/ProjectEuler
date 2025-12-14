use std::{char::from_digit, collections::HashMap};
fn main() {
    let n = 1000;
    let mut curr_max = 0;
    let mut res = 0;
    for i in 2..n {
        let mut details = String::from("0.");
        let mut curr_numbers = HashMap::new();
        let mut start = 1;
        let mut counter = 0;
        loop {
            let q = start * 10 / i;
            details.push(from_digit(q, 10).unwrap());
            if q == 0 {
                start *= 10;
            } else {
                let r = start * 10 % i;
                start = r;
                if let Some(num) = curr_numbers.get(&r) {
                    let cycle = counter - num;
                    details.push_str(&format!(" - {}", cycle));
                    if cycle > curr_max {
                        curr_max = cycle;
                        res = i;
                    }
                    break;
                } else {
                    curr_numbers.insert(r, counter);
                }
                if r == 0 {
                    break;
                }
            }
            counter += 1;
        }
        println!("1/{i} = {details}")
    }
    println!("Res: {res}");
}
