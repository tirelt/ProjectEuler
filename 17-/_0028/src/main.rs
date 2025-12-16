fn main() {
    /* The rucurrence relations are
    - top right diagonal = Un = Un-1 + n * 8
    - top left diagonal = Un = Un-1 + n * 8 - 2
    - bottom left diagonal = Un = Un-1 + n * 8  - 4
    - bottom right diagonal = Un = Un-1 + n * 8  - 6
    So the of the ith numbers on the 4 diagonal is 1 if n = 0 and 16n^2 +4n + 4 otherwise
    Then we can sum this for n = 1 to (size-1)/2 to get the closed formula
    */
    let size: u64 = 1001;
    let n = (size - 1) / 2;
    let res = 1 + 16 * (n * (n + 1) * (2 * n + 1)) / 6 + 4 * n * (n + 1) / 2 + 4 * n;
    println!("Res: {res}");
}
