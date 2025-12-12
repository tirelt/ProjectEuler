fn main() {
    let mut curr = 1;
    let mut fact = vec![1; 11];
    for i in 1..=10 {
        curr = curr * i;
        fact[i] = curr;
    }
    println!("TEst");
}
