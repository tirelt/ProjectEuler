fn generate_triangle(n: &mut u64) -> u64 {
    let res = *n * (*n + 1) / 2;
    *n += 1;
    res
}
fn generate_pentagonal(n: &mut u64) -> u64 {
    let res = *n * (*n * 3 - 1) / 2;
    *n += 1;
    res
}
fn generate_hexagonal(n: &mut u64) -> u64 {
    let res = *n * (*n * 2 - 1);
    *n += 1;
    res
}
fn main() {
    let mut n_triangle = 1;
    let mut n_pentagonal = 1;
    let mut n_hexagonal = 1;
    let mut triangle = generate_triangle(&mut n_triangle);
    let mut pentagonal = generate_pentagonal(&mut n_pentagonal);
    let mut hexagonal = generate_hexagonal(&mut n_hexagonal);
    let mut counter = 0;
    while counter < 3 {
        match (triangle, pentagonal, hexagonal) {
            (t, p, h) if t == p && t == h => {
                counter += 1;
                triangle = generate_triangle(&mut n_triangle)
            }
            (t, p, h) if t <= p && t <= h => triangle = generate_triangle(&mut n_triangle),
            (t, p, h) if p <= t && p <= h => pentagonal = generate_pentagonal(&mut n_pentagonal),
            (t, p, h) if h <= t && h <= p => hexagonal = generate_hexagonal(&mut n_hexagonal),
            (_, _, _) => panic!("shouldn't go here"),
        }
    }
    println!("Res: {pentagonal}");
}
