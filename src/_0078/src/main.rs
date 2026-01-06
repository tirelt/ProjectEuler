use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut p_n = vec![1, 1, 2]; // number partions of 0, 1, 2
    while *p_n.last().unwrap() != 0 {
        let n = p_n.len();
        let mut new_last_value = 0;
        let mut k = 1;
        let mut mul = 1;
        loop {
            let i = n as i32 - (k * (3 * k - 1) / 2);
            if i < 0 {
                break;
            }
            new_last_value += mul * p_n[i as usize];
            mul *= -1;
            k += 1;
        }
        k = 1;
        mul = 1;
        loop {
            let i = n as i32 - (k * (3 * k + 1) / 2);
            if i < 0 {
                break;
            }
            new_last_value += mul * p_n[i as usize];
            mul *= -1;
            k += 1;
        }
        p_n.push(new_last_value % 1_000_000);
    }
    println!("Res: {}", p_n.len() - 1);
    println!("Duration: {}ms", start.elapsed().as_millis());
}
