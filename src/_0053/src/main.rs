use std::ops::Add;
enum Num {
    Small(i32),
    Big,
}

impl Add for &Num {
    type Output = Num;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Num::Small(a), Num::Small(b)) => {
                if *a > 1_000_000 - *b {
                    Num::Big
                } else {
                    Num::Small(a + b)
                }
            }
            (_, _) => Num::Big,
        }
    }
}
fn main() {
    let mut pascal = vec![vec![Num::Small(1)]];
    let mut res = 0;
    for n in 1..=100 as usize {
        let prev = &pascal[n - 1];
        let mut curr = vec![Num::Small(1)];
        for k in 1..=n as usize {
            let res_add = &prev[k - 1] + if k < n { &prev[k] } else { &Num::Small(0) };
            if let Num::Big = res_add {
                res += 1;
            }
            curr.push(res_add);
        }
        pascal.push(curr);
    }
    println!("Res: {res}");
}
