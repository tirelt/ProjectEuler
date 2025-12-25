use std::ops::{Add, Mul};
pub struct BigNum {
    digits: Vec<u64>,
}
impl BigNum {
    pub fn new() -> Self {
        BigNum { digits: vec![0] }
    }
    pub fn new_from_u64(a: u64) -> Self {
        let mut digits = Vec::new();
        let mut a_ = a;
        while a_ > 0 {
            digits.push(a_ % 10);
            a_ /= 10;
        }
        BigNum { digits }
    }
}
impl Add for &BigNum {
    type Output = BigNum;
    fn add(self, other: Self) -> Self::Output {
        let mut res: Vec<u64>;
        let mut small = &other.digits;
        if self.digits.len() > other.digits.len() {
            res = self.digits.clone();
        } else {
            res = other.digits.clone();
            small = &self.digits;
        }
        let mut keep = 0;
        for i in 0..small.len() {
            keep += res[i] + small[i];
            res[i] = keep % 10;
            keep /= 10;
        }
        for i in small.len()..res.len() {
            keep += res[i];
            res[i] += keep % 10;
        }
        while keep > 0 {
            res.push(keep % 10);
            keep /= 10;
        }
        BigNum { digits: res }
    }
}
impl Mul<u64> for &BigNum {
    type Output = BigNum;
    fn mul(self, c: u64) -> Self::Output {
        let mut keep = 0;
        let mut res = self.digits.clone();
        for v in res.iter_mut() {
            let new_v = *v * c + keep;
            keep = new_v / 10;
            *v = new_v % 10;
        }
        while keep > 0 {
            res.push(keep % 10);
            keep = keep / 10;
        }
        BigNum { digits: res }
    }
}
impl Mul<&BigNum> for u64 {
    type Output = BigNum;
    fn mul(self, rhs: &BigNum) -> Self::Output {
        rhs * self
    }
}
