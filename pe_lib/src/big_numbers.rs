use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::{Ordering, PartialOrd};
use std::collections::VecDeque;
use std::ops::{Add, Mul, Sub};

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
    pub fn new_from_vec(digits: VecDeque<u64>) -> Self {
        BigNum { digits }
    }
    pub fn len(&self) -> usize {
        self.digits.len()
    }
    pub fn mult_by_10_pow(&mut self, n: usize) {
        for _ in 0..n {
            self.digits.push_front(0);
        }
    }
    pub fn div_by_10_pow(&mut self, n: usize) {
        for _ in 0..n {
            self.digits.pop_front();
        }
    }
    pub fn div(&self, other: &Self) -> (Self, Self) {
        if self < other {
            return (BigNum::new_from_u64(0), self.clone());
        }
        let mut zeros_added = 0;
        let mut curr_divisor = other.clone();
        if self.len() > other.len() + 1 {
            zeros_added = (self.len() - (other.len())) as i32;
            curr_divisor.mult_by_10_pow(zeros_added as usize);
        }
        let mut quotient = VecDeque::new();
        let mut curr_num = self.clone();
        while zeros_added >= 0 {
            let mut c: u64 = 0;
            while curr_num > curr_divisor {
                curr_num = &curr_num - &curr_divisor;
                c += 1;
            }
            curr_divisor.div_by_10_pow(1);
            zeros_added -= 1;
            quotient.push_back(c);
        }
        (BigNum::new_from_vec(quotient), curr_num)
    }
    pub fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }
}
impl PartialEq for BigNum {
    fn eq(&self, other: &Self) -> bool {
        self.digits == other.digits
    }
}
impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.len(), other.len()) {
            (a, b) if a < b => Some(Less),
            (a, b) if a > b => Some(Greater),
            (a, _) => {
                for i in (0..a).rev() {
                    if self.digits[i] < other.digits[i] {
                        return Some(Less);
                    }
                    if self.digits[i] > other.digits[i] {
                        return Some(Greater);
                    }
                }
                Some(Equal)
            }
        }
    }
}
impl Sub for &BigNum {
    type Output = BigNum;
    // only for self >= other
    fn sub(self, other: Self) -> Self::Output {
        let mut new_digits: VecDeque<u64> = VecDeque::new();
        let mut carry = 0;
        for i in 0..other.len() {
            let mut left = self.digits[i];
            let right = other.digits[i];
            if left < right + carry {
                left += 10 - carry;
                carry = 1;
            } else {
                left -= carry;
                carry = 0;
            }
            left -= right;
            new_digits.push_back(left);
        }
        for i in other.len()..self.len() {
            new_digits.push_back(self.digits[i] - carry);
            if carry != 0 {
                carry = 0;
            }
        }
        let mut j = new_digits.len() as i32 - 1;
        while j >= 0 && new_digits[j as usize] == 0 {
            j -= 1;
        }
        if j < 0 {
            return BigNum::new_from_u64(0);
        } else {
            return BigNum::new_from_vec(new_digits.into_iter().take(j as usize + 1).collect());
        }
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
