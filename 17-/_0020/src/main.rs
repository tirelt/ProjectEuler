fn main() {
    let number_digit: f64 = (1..=100).map(|x| (x as f64).ln()/(10 as f64).ln()).sum();
    println!("Approx numver of digit of 100! {number_digit}");
    let mut digit: [u32;160]= [0;160];
    digit[0] = 1;
    for i in 1..=100 {
        for j in 0..digit.len() {
            if digit[j] > 0 {
                let mut temp: u32 = i * digit[j] as u32;
                let hundred = temp / 100;
                temp = temp % 100;
                let dozen = temp / 10;
                temp = temp % 10;
                digit[j] = temp;
                digit[j+1] += dozen;
                digit[j+2] += hundred;
            }
        }
    }
}

