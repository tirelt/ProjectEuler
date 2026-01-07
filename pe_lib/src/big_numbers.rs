use std::collections::VecDeque;
use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
pub struct BigNum {
    pub digits: VecDeque<u64>,
}

impl BigNum {
    pub fn new() -> Self {
        BigNum {
            digits: VecDeque::from([0]),
        }
    }
    pub fn new_from_u64(a: u64) -> Self {
        let mut digits = VecDeque::new();
        let mut a_ = a;
        while a_ > 0 {
            digits.push_back(a_ % 10);
            a_ /= 10;
        }
        BigNum { digits }
    }
}
impl Add for &BigNum {
    type Output = BigNum;
    fn add(self, other: Self) -> Self::Output {
        let mut res: VecDeque<u64>;
        let mut small = &other.digits;
        if self.digits.len() > other.digits.len() {
            res = self.digits.clone();
        } else {
            res = other.digits.clone();
            small = &self.digits;
        }
        let mut keep = 0;
        for i in 0..small.len() {
            res[i] += small[i] + keep;
            keep = res[i] / 10;
            res[i] %= 10;
        }
        if keep > 0 {
            for i in small.len()..res.len() {
                res[i] += keep;
                keep = res[i] / 10;
                res[i] %= 10;
            }
            while keep > 0 {
                res.push_back(keep % 10);
                keep /= 10;
            }
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
            res.push_back(keep % 10);
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
