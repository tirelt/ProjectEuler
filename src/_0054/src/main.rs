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
        let mut value = ite.next().unwrap();
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
    fn value(&self) -> i32 {
        let mut flush = false;
        for (_, count) in self.colors.iter() {
            if *count == 5 {
                flush = true;
            }
        }
        let mut straight = true;
        let mut prev_v = 0;
        let mut four = 0;
        let mut three = 0;
        let mut pairs = Vec::new();
        let mut highest = 0;
        for (val, count) in self.values.iter() {
            match count {
                4 => four = *val,
                3 => three = *val,
                2 => pairs.push(*val),
                _ => (),
            };
            if *val > highest {
                highest = *val;
            }
            if prev_v == 0 {
                prev_v = *val;
            } else {
                if prev_v + 1 != *val {
                    straight = false;
                }
                prev_v = *val;
            }
        }
        pairs.sort_by(|a, b| b.cmp(a));
        match (flush, straight, four, three, pairs) {
            (true, true, ..) => 90_000 + highest,
            (_, _, x, ..) if x > 0 => 80_000 + x,
            (true, ..) => 70_00 + highest,
            (_, true, ..) => 60_00 + highest,
            (_, _, _, x, p) if x > 0 && p.len() > 0 => 50_000 + x * 10 + p[0],
            (_, _, _, x, _) if x > 0 => 40_000 + x * 100,
            (_, _, _, _, p) if p.len() > 0 => {
                30_000 + p[0] * 100 + if p.len() > 1 { p[1] } else { 0 }
            }
            (_, _, _, _, _) => 0,
        }
    }
}

fn main() {
    let file = fs::read_to_string("poker.txt").unwrap();
    let mut res = 0;
    for line in file.lines() {
        let mut counter = 0;
        let mut h1 = Hand::new();
        let mut h2 = Hand::new();
        for card in line.split(' ') {
            if counter < 5 {
                h1.push(card);
            } else {
                h2.push(card);
            }
            counter += 1;
        }
        let value_1 = h1.value();
        let value_2 = h2.value();
        if value_1 > value_2 {
            res += 1;
        } else if value_2 > value_1 {
            continue;
        } else {
            let values_1: Vec<i32> = h1.values.iter().rev().map(|(&v, _)| v).collect();
            let values_2: Vec<i32> = h2.values.iter().rev().map(|(&v, _)| v).collect();
            for i in 0..values_1.len() {
                if values_1[i] > values_2[i] {
                    res += 1;
                    break;
                }
                if values_1[i] < values_2[i] {
                    break;
                }
                println!("tie");
            }
            //println!("debug");
        }
    }
    println!("Res: {res}");
}
