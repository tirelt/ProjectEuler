fn next_fib(first: Vec<i32>, mut second: Vec<i32>) -> Vec<i32> {
    for i in 0..first.len() {
        let val = first[i] + second[i];
        second[i] = val % 10;
        let add_to_next = val / 10;
        if add_to_next > 0 {
            if i < second.len() - 1 {
                second[i + 1] += add_to_next;
            } else {
                second.push(add_to_next);
            }
        }
    }
    second
}
fn main() {
    let n_digit = 1000;
    let mut first = vec![1];
    let mut second = vec![1];
    let mut counter = 2;
    while second.len() < n_digit {
        let temp = second.clone();
        second = next_fib(first, second);
        first = temp;
        counter += 1;
    }
    println!("Res: {counter}");
}
