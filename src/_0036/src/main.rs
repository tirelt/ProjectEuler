fn main() {
    let mut res = 0;
    for i in 1..1_000_000 {
        let s = i.to_string();
        let s_rev: String = s.chars().rev().collect();
        if s == s_rev {
            let binary = format!("{:b}", i);
            let binary_rev: String = binary.chars().rev().collect();
            if binary == binary_rev {
                res += i;
            }
        }
    }
    println!("Res: {res}");
}
