fn get_fact(n: u64, table: &mut Vec<u64>) {
    match n {
        0 => table.push(1),
        k => {
            get_fact(k - 1, table);
            table.push(table.last().unwrap() * k);
        }
    }
}
fn main() {
    let target: u64 = 1_000_000;
    let mut table = Vec::new();
    let test = get_fact(10, &mut table);
    let first = target / &table[9];
    println!("TEst");
}
