///! Simple tool to conver from a give base into any other base
///! For example
/// 1. From base 2 to/from base 10
/// 2. From base 2 to/from base 16
/// 3. From base 16 to base base 10
#[allow(dead_code, unused)]
use std::collections::HashMap;

use num::ToPrimitive;

const BASE_16: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];
const BASE_10: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const BASE_2: &[&str] = &["0", "1"];

fn main() {
    let base_2 = Base::new_base(CommonBase::Base2);
    let base_10 = Base::new_base(CommonBase::Base10);
    let base_16 = Base::new_base(CommonBase::Base16);

    println!("[base 2] {:?}", base_2);
    println!("[base 10] {:?}", base_10);
    println!("[base 16] {:?}", base_16);

    // b2 🔛 b10
    let _10b2 = base_2.from_b10(10);
    println!("b2(10) -> {}", _10b2.join(""));
    let _10b10 = base_2.into_b10(&_10b2);
    println!("b10({:?}) -> {}", _10b2, _10b10);
    // b10 🔛 b16
    let _175_b16 = base_16.from_b10(175);
    println!("b16(175) -> 0x{}", _175_b16.join(""));
    let _175_b10 = base_10.from_base(&_175_b16, &base_16);
    println!("b10({:?}) -> {}", _175_b16, _175_b10.join(""));
    // b2 🔛 b16
    let _175_b16 = base_16.from_base(&_10b2, &base_2);
    println!("b16({:?}) -> 0x{}", _10b2, _175_b16.join(""));
    let _175_b12 = base_2.from_base(&_175_b16, &base_16);
    println!("b2({:?}) -> {}", _175_b16, _175_b12.join(""));
}

/// Convert base 2 to base 16 or base 10
#[derive(Debug)]
struct Base<'a> {
    base: usize,
    digits: Vec<&'a str>,
}

impl<'a> Base<'a> {
    fn new(digits: Vec<&'a str>) -> Self {
        Self {
            base: digits.len(),
            digits,
        }
    }

    fn new_base(base: CommonBase) -> Self {
        let digits = match base {
            CommonBase::Base2 => BASE_2,
            CommonBase::Base10 => BASE_10,
            CommonBase::Base16 => BASE_16,
        };

        Self {
            base: digits.len(),
            digits: digits.to_vec(),
        }
    }

    fn from_b10<N: ToPrimitive>(&self, num: N) -> Vec<&'a str> {
        let mut res = Vec::new();
        let mut curr_num = num.to_usize().unwrap();
        loop {
            let rem = curr_num % self.base;
            res.push(self.digits[rem]);
            curr_num = curr_num / self.base;

            if curr_num == 0 {
                break;
            }
        }
        res.reverse();
        res
    }

    fn into_b10(&self, num: &[&str]) -> usize {
        let mut map = HashMap::new();
        self.digits.iter().enumerate().for_each(|(idx, &num)| {
            map.insert(num, idx);
        });
        let mut res = 0;
        num.iter().rev().enumerate().for_each(|(idx, &n)| {
            let num = map[n];
            res += num * self.base.pow(idx as u32)
        });

        res
    }

    fn from_base(&self, num: &[&str], base: &Base) -> Vec<&str> {
        let mut map = HashMap::new();
        base.digits.iter().enumerate().for_each(|(idx, &num)| {
            map.insert(num, idx);
        });
        let mut base_10_value = 0;

        num.iter().rev().enumerate().for_each(|(idx, &d)| {
            let num = map.get(d).expect("Invalid encoding").clone();
            base_10_value += num * base.base.pow(idx as u32);
        });

        self.from_b10(base_10_value)
    }
}

enum CommonBase {
    Base10,
    Base2,
    Base16,
}
