// Copyright 2018 Đỗ Hoàng Anh Duy.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{fmt, mem};

#[derive(Debug)]
pub struct Dumped {
    pub sign: char,
    pub exp_s: String,
    pub val: String,
    pub exp_biased: i32,
    pub exp: i32,
    pub value: String,
}

pub trait Binaryer
where
    Self: Sized + fmt::LowerExp,
{
    const EXP_SIZE: usize;
    const BIT_SIZE: usize = mem::size_of::<Self>() * 8;

    fn bias() -> i32 {
        (1 << (Self::EXP_SIZE - 1)) - 1
    }

    fn dump(&self) -> Dumped {
        let s = (0..Self::BIT_SIZE)
            .rev()
            .map(|i| self.get_bit(i))
            .collect::<Vec<_>>();
        let exp = s[1..(1 + Self::EXP_SIZE)].iter().collect::<String>();
        let exp_val = i32::from_str_radix(&exp, 2).unwrap();
        Dumped {
            sign: s[0],
            exp_s: exp,
            val: s[(1 + Self::EXP_SIZE)..].iter().collect::<String>(),
            exp_biased: exp_val,
            exp: exp_val - Self::bias(),
            value: format!("{:e}", self),
        }
    }

    fn build(bits: &str) -> Result<Self, &'static str> {
        let bits = bits.trim();
        let iter = bits.chars().filter(|c| !c.is_whitespace());
        if iter.clone().count() > Self::BIT_SIZE {
            Err("Input is too long")
        } else if iter.clone().any(|c| c != '0' && c != '1') {
            Err("Invalid character in input")
        } else {
            let mut result = unsafe { mem::zeroed::<Self>() };
            iter.enumerate()
                .filter(|&(_, c)| c == '1')
                .for_each(|(i, _)| result.set_bit(Self::BIT_SIZE - 1 - i));
            Ok(result)
        }
    }

    fn get_bit(&self, k: usize) -> char;
    fn set_bit(&mut self, k: usize);
}

impl Binaryer for f32 {
    const EXP_SIZE: usize = 8;

    fn get_bit(&self, k: usize) -> char {
        let ptr = self as *const f32 as *const u32;
        unsafe { ['0', '1'][(0 != *ptr & (1u32 << k)) as usize] }
    }

    fn set_bit(&mut self, k: usize) {
        let ptr = self as *mut f32 as *mut u32;
        unsafe {
            *ptr |= 1u32 << k;
        }
    }
}

impl Binaryer for f64 {
    const EXP_SIZE: usize = 11;

    fn get_bit(&self, k: usize) -> char {
        let ptr = self as *const f64 as *const u64;
        unsafe { ['0', '1'][(0 != *ptr & (1u64 << k)) as usize] }
    }

    fn set_bit(&mut self, k: usize) {
        let ptr = self as *mut f64 as *mut u64;
        unsafe {
            *ptr |= 1u64 << k;
        }
    }
}
