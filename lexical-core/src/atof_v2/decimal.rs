#![allow(unused)]   // TODO(ahuszagh) Remove...

use crate::util::*;

#[derive(Clone)]
pub struct Decimal {
    pub num_digits: usize,
    pub decimal_point: i32,
    pub negative: bool,
    pub truncated: bool,
    pub digits: [u8; Self::MAX_DIGITS],
}

impl Decimal {
    pub const MAX_DIGITS: usize = 768;
    pub const MAX_DIGITS_WITHOUT_OVERFLOW: usize = 19;
    pub const DECIMAL_POINT_RANGE: i32 = 2047;

    // TODO(ahuszagh) Could be faster...
    #[inline]
    pub fn try_add_digit(&mut self, digit: u8) {
        if self.num_digits < Self::MAX_DIGITS {
            self.digits[self.num_digits] = digit;
        }
        self.num_digits += 1;
    }

    #[inline]
    pub fn trim(&mut self) {
        while self.num_digits != 0 && self.digits[self.num_digits - 1] == 0 {
            self.num_digits -= 1;
        }
    }

    #[inline]
    pub fn round(&self) -> u64 {
        // TODO(ahuszagh) Needs to support other radixes...
        todo!()
//        if self.num_digits == 0 || self.decimal_point < 0 {
//            return 0;
//        } else if self.decimal_point > 18 {
//            return 0xFFFF_FFFF_FFFF_FFFF_u64;
//        }
//        let dp = self.decimal_point as usize;
//        let mut n = 0_u64;
//        for i in 0..dp {
//            n *= 10;
//            if i < self.num_digits {
//                n += self.digits[i] as u64;
//            }
//        }
//        let mut round_up = false;
//        if dp < self.num_digits {
//            round_up = self.digits[dp] >= 5;
//            if self.digits[dp] == 5 && dp + 1 == self.num_digits {
//                round_up = self.truncated || ((dp != 0) && (1 & self.digits[dp - 1] != 0))
//            }
//        }
//        if round_up {
//            n += 1;
//        }
//        n
    }

    #[inline]
    pub fn left_shift(&mut self, shift: usize) {
        // TODO(ahuszagh) Needs to support other radixes...
        todo!()
//        if self.num_digits == 0 {
//            return;
//        }
//        let num_new_digits = number_of_digits_decimal_left_shift(self, shift);
//        let mut read_index = self.num_digits;
//        let mut write_index = self.num_digits + num_new_digits;
//        let mut n = 0_u64;
//        while read_index != 0 {
//            read_index -= 1;
//            write_index -= 1;
//            n += (self.digits[read_index] as u64) << shift;
//            let quotient = n / 10;
//            let remainder = n - (10 * quotient);
//            if write_index < Self::MAX_DIGITS {
//                self.digits[write_index] = remainder as u8;
//            } else if remainder > 0 {
//                self.truncated = true;
//            }
//            n = quotient;
//        }
//        while n > 0 {
//            write_index -= 1;
//            let quotient = n / 10;
//            let remainder = n - (10 * quotient);
//            if write_index < Self::MAX_DIGITS {
//                self.digits[write_index] = remainder as u8;
//            } else if remainder > 0 {
//                self.truncated = true;
//            }
//            n = quotient;
//        }
//        self.num_digits += num_new_digits;
//        if self.num_digits > Self::MAX_DIGITS {
//            self.num_digits = Self::MAX_DIGITS;
//        }
//        self.decimal_point += num_new_digits as i32;
//        self.trim();
    }

    #[inline]
    pub fn right_shift(&mut self, shift: usize) {
//        let mut read_index = 0;
//        let mut write_index = 0;
//        let mut n = 0_u64;
//        while (n >> shift) == 0 {
//            if read_index < self.num_digits {
//                n = (10 * n) + self.digits[read_index] as u64;
//                read_index += 1;
//            } else if n == 0 {
//                return;
//            } else {
//                while (n >> shift) == 0 {
//                    n *= 10;
//                    read_index += 1;
//                }
//                break;
//            }
//        }
//        self.decimal_point -= read_index as i32 - 1;
//        if self.decimal_point < -Self::DECIMAL_POINT_RANGE {
//            self.num_digits = 0;
//            self.decimal_point = 0;
//            self.negative = false;
//            self.truncated = false;
//            return;
//        }
//        let mask = (1_u64 << shift) - 1;
//        while read_index < self.num_digits {
//            let new_digit = (n >> shift) as u8;
//            n = (10 * (n & mask)) + self.digits[read_index] as u64;
//            read_index += 1;
//            self.digits[write_index] = new_digit;
//            write_index += 1;
//        }
//        while n > 0 {
//            let new_digit = (n >> shift) as u8;
//            n = 10 * (n & mask);
//            if write_index < Self::MAX_DIGITS {
//                self.digits[write_index] = new_digit;
//                write_index += 1;
//            } else if new_digit > 0 {
//                self.truncated = true;
//            }
//        }
//        self.num_digits = write_index;
//        self.trim();
    }
}

// TODO(ahuszagh) Should just use Number....
#[inline]
pub(crate) fn parse_decimal<'a, Iter>(iter: Iter) -> Decimal
where
    Iter: ContiguousIterator<'a, u8>,
{
    // can't fail since it follows a call to parse_number
    // TODO(ahuszagh) Implement...
    todo!()
}
