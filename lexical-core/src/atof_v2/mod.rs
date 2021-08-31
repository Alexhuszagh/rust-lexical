// TODO(ahuszagh) Document...

#![allow(unused)]   // TODO(ahuszagh) Remove...

mod binary;
mod common;
mod decimal;
mod float;
mod number;
mod options;
mod parse;
mod simple;
mod starts;
mod table;
mod table_decimal;
#[cfg(feature = "radix")]
mod table_radix;

pub use self::parse::parse_float;
