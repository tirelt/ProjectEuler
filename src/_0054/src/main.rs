use std::collections::BTreeMap;
use std::fs;

struct Hand {
    cards: Vec<String>,
    colors: BTreeMap<char, i32>,
    values: BTreeMap<i32, i32>,
}
impl Hand {
    fn new() -> Self {
        Hand {
            cards: Vec::new(),
            colors: BTreeMap::new(),
            values: BTreeMap::new(),
        }
    }
    fn push(&mut self, card: &str) {
        self.cards.push(card.to_owned());
        let mut ite = card.chars();
        let value = ite.next().unwrap();
        let color = ite.next().unwrap();
        //println!("{value}-{color}");
        let value = if !value.is_digit(10) {
            match value {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                x => panic!("Shouldn't go here, {x}"),
            }
        } else {
            value.to_string().parse().unwrap()
        };
        *self.colors.entry(color).or_insert(0) += 1;
        *self.values.entry(value).or_insert(0) += 1;
    }
    fn value(&self) -> (i32, Vec<i32>) {
        let mut flush = false;
        for (_, count) in self.colors.iter() {
            if *count == 5 {
                flush = true;
            }
        }
        let mut max_count = 0;
        let mut four = 0;
        let mut three = 0;
        let mut pairs = Vec::new();
        let mut highest = 0;
        let mut values = Vec::new();
        for (val, count) in self.values.iter() {
            values.push(*val);
            match count {
                4 => four = *val,
                3 => three = *val,
                2 => pairs.push(*val),
                _ => (),
            };
            if *val > highest {
                highest = *val;
            }
            if *count > max_count {
                max_count = *count;
            }
        }
        let mut straight = false;
        if max_count == 1 {
            straight = true;
            for i in 1..values.len() {
                let diff = (values[i] - values[i - 1]).abs();
                if diff > 1 {
                    straight = false;
                    break;
                }
            }
        }
        values.sort_by(|a, b| b.cmp(a));
        if values == vec![14, 5, 4, 3, 2] {
            straight = true;
            highest = 5;
        }
        pairs.sort_by(|a, b| b.cmp(a));
        let value_hand = match (flush, straight, four, three, pairs) {
            (true, true, ..) => 90_000 + highest,
            (_, _, x, ..) if x > 0 => 80_000 + x,
            (true, ..) => 70_000 + highest,
            (_, true, ..) => 60_000 + highest,
            (_, _, _, x, p) if x > 0 && p.len() > 0 => 50_000 + x * 100 + p[0],
            (_, _, _, x, _) if x > 0 => 40_000 + x * 100,
            (_, _, _, _, p) if p.len() == 2 => 30_000 + p[0] * 100 + p[1],
            (_, _, _, _, p) if p.len() == 1 => 20_000 + p[0] * 100,
            (_, _, _, _, _) => 0,
        };
        (value_hand, values)
    }
}

fn main() {
    let solution_file = fs::read_to_string("correct_winners.txt").unwrap();
    let mut solutions = Vec::new();
    for line in solution_file.lines() {
        let temp: i32 = line.parse().unwrap();
        solutions.push(temp);
    }
    let file = fs::read_to_string("poker.txt").unwrap();
    let mut res = 0;
    let mut n_hand = 0;
    for line in file.lines() {
        let mut counter = 0;
        let mut h1 = Hand::new();
        let mut h2 = Hand::new();
        let mut winner = 2;
        for card in line.split(' ') {
            if counter < 5 {
                h1.push(card);
            } else {
                h2.push(card);
            }
            counter += 1;
        }
        let (value_1, values_1) = h1.value();
        let (value_2, values_2) = h2.value();
        if value_1 > value_2 {
            winner = 1;
        } else if value_2 == value_1 {
            for i in 0..values_1.len() {
                if values_1[i] > values_2[i] {
                    winner = 1;
                    break;
                }
                if values_1[i] < values_2[i] {
                    break;
                }
                println!("tie");
            }
        }
        if winner == 1 {
            res += 1;
        }
        if winner != solutions[n_hand] {
            println!("debug");
        }
        n_hand += 1;
    }

    println!("Res: {res}");
}
