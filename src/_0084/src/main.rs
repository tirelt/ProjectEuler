use rand::prelude::*;
use std::collections::HashMap;

fn next_position(
    curr: usize,
    mut consecutive_double: u64,
    mut cc_n: usize,
    mut ch_n: usize,
    double_count: &mut u64,
    squares: &Vec<String>,
    square_map: &HashMap<String, usize>,
    cc_cards: &Vec<Option<String>>,
    ch_cards: &Vec<Option<String>>,
    rng: &mut ThreadRng,
) -> (usize, u64, usize, usize) {
    let throw_1: usize = rng.random_range(1..=4);
    let throw_2: usize = rng.random_range(1..=4);
    let throw = throw_1 + throw_2;
    if throw_1 == throw_2 {
        consecutive_double += 1;
        *double_count += 1;
    } else {
        consecutive_double = 0;
    }
    if consecutive_double >= 3 {
        return (square_map["JAIL"], 0, cc_n, ch_n);
    }
    let mut new_pos = (curr + throw) % squares.len();
    match squares[new_pos].as_str() {
        "G2J" => new_pos = square_map["JAIL"],
        "CC1" | "CC2" | "CC3" => {
            let cc = &cc_cards[cc_n];
            if let Some(cc) = cc {
                new_pos = square_map[cc];
            }
            cc_n = (cc_n + 1) % cc_cards.len();
        }
        "CH1" | "CH2" | "CH3" => {
            let ch = &ch_cards[ch_n];
            if let Some(ch) = ch {
                match ch.as_str() {
                    "RX" => {
                        while !squares[new_pos].starts_with("R") {
                            new_pos = (new_pos + 1) % 40;
                        }
                    }
                    "UX" => {
                        while !squares[new_pos].starts_with("U") {
                            new_pos = (new_pos + 1) % 40;
                        }
                    }
                    "3B" => {
                        new_pos -= 3; // lowest chance is at 7 so no need to modulo
                        if squares[new_pos].starts_with("CC") {
                            let cc = &cc_cards[cc_n];
                            if let Some(cc) = cc {
                                new_pos = square_map[cc];
                            }
                            cc_n = (cc_n + 1) % cc_cards.len();
                        }
                    }
                    x => new_pos = square_map[x],
                }
            }
            ch_n = (ch_n + 1) % ch_cards.len();
        }
        _ => (),
    };
    (new_pos, consecutive_double, cc_n, ch_n)
}
fn main() {
    let mut rng = rand::rng();
    let squares = vec![
        String::from("GO"),
        String::from("A1"),
        String::from("CC1"),
        String::from("A2"),
        String::from("T1"),
        String::from("R1"),
        String::from("B1"),
        String::from("CH1"),
        String::from("B2"),
        String::from("B3"),
        String::from("JAIL"),
        String::from("C1"),
        String::from("U1"),
        String::from("C2"),
        String::from("C3"),
        String::from("R2"),
        String::from("D1"),
        String::from("CC2"),
        String::from("D2"),
        String::from("D3"),
        String::from("FP"),
        String::from("E1"),
        String::from("CH2"),
        String::from("E2"),
        String::from("E3"),
        String::from("R3"),
        String::from("F1"),
        String::from("F2"),
        String::from("U2"),
        String::from("F3"),
        String::from("G2J"),
        String::from("G1"),
        String::from("G2"),
        String::from("CC3"),
        String::from("G3"),
        String::from("R4"),
        String::from("CH3"),
        String::from("H1"),
        String::from("T2"),
        String::from("H2"),
    ];
    let mut square_map = HashMap::new();
    for i in 0..squares.len() {
        square_map.insert(squares[i].clone(), i);
    }
    let mut counter = vec![0; squares.len()];
    let mut cc_cards = vec![None; 14];
    cc_cards.push(Some(String::from("GO")));
    cc_cards.push(Some(String::from("JAIL")));
    cc_cards.shuffle(&mut rng);

    let mut ch_cards = vec![None; 6];
    ch_cards.push(Some(String::from("GO")));
    ch_cards.push(Some(String::from("JAIL")));
    ch_cards.push(Some(String::from("C1")));
    ch_cards.push(Some(String::from("E3")));
    ch_cards.push(Some(String::from("H2")));
    ch_cards.push(Some(String::from("R1")));
    ch_cards.push(Some(String::from("RX")));
    ch_cards.push(Some(String::from("RX")));
    ch_cards.push(Some(String::from("UX")));
    ch_cards.push(Some(String::from("3B")));
    ch_cards.shuffle(&mut rng);

    let n_throw = 10_000_000;
    let mut curr = 0;
    let mut cc_n = 0;
    let mut ch_n = 0;
    let mut consecutive_double = 0;
    let mut double_count: u64 = 0;
    counter[0] += 1;
    for i in 0..n_throw {
        if i % 100_000 == 0 {
            // cc_cards.shuffle(&mut rng);
            //ch_cards.shuffle(&mut rng);
        }
        (curr, consecutive_double, cc_n, ch_n) = next_position(
            curr,
            consecutive_double,
            cc_n,
            ch_n,
            &mut double_count,
            &squares,
            &square_map,
            &cc_cards,
            &ch_cards,
            &mut rng,
        );
        counter[curr] += 1;
    }
    let mut probas = Vec::new();
    for i in 0..squares.len() {
        probas.push((
            i,
            squares[i].clone(),
            counter[i] as f64 / n_throw as f64 * 100.0,
        ));
    }
    probas.sort_by(|x, y| (y.2).partial_cmp(&x.2).unwrap());
    println! {"Res: {:?}",&probas[0..3]}
}
