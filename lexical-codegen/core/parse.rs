#[no_mangle]
pub fn parse_u8(data: &str) -> Result<u8, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_u16(data: &str) -> Result<u16, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_u32(data: &str) -> Result<u32, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_u64(data: &str) -> Result<u64, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_u128(data: &str) -> Result<u128, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_usize(data: &str) -> Result<usize, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_i8(data: &str) -> Result<i8, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_i16(data: &str) -> Result<i16, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_i32(data: &str) -> Result<i32, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_i64(data: &str) -> Result<i64, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_i128(data: &str) -> Result<i128, std::num::ParseIntError> {
    data.parse()
}

#[no_mangle]
pub fn parse_isize(data: &str) -> Result<isize, std::num::ParseIntError> {
    data.parse()
}


pub fn main() {
    println!("{}", parse_u8("0").unwrap());
    println!("{}", parse_u16("0").unwrap());
    println!("{}", parse_u32("0").unwrap());
    println!("{}", parse_u64("0").unwrap());
    println!("{}", parse_u128("0").unwrap());
    println!("{}", parse_usize("0").unwrap());
    println!("{}", parse_i8("0").unwrap());
    println!("{}", parse_i16("0").unwrap());
    println!("{}", parse_i32("0").unwrap());
    println!("{}", parse_i64("0").unwrap());
    println!("{}", parse_i128("0").unwrap());
    println!("{}", parse_isize("0").unwrap());
}
