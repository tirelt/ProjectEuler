use pe_lib::primes::gcd;

fn main() {
    let max_l: u64 = 1_500_000;
    let sqrt = (max_l as f64).sqrt() as u64;
    let mut possible_length = Vec::new();
    for n in 1..=sqrt {
        for m in (1 + n)..=sqrt {
            if n == 2 && m == 3 {
                println!("");
            }
            if (n + m) % 2 == 1 && gcd(n, m) == 1 {
                let a = m.pow(2) - n.pow(2);
                let b = 2 * m * n;
                let c = m.pow(2) + n.pow(2);
                let length = a + b + c;
                if length <= max_l {
                    possible_length.push(length);
                }
            }
        }
    }
    //possible_length.sort();
    let mut res_all = vec![0; max_l as usize + 1];
    for l in possible_length {
        let mut c = 1;
        while c * l <= max_l {
            res_all[(c * l) as usize] += 1;
            c += 1;
        }
    }
    let res: usize = res_all.iter().filter(|&x| *x == 1).count();
    println!("Res: {}", res);
}
