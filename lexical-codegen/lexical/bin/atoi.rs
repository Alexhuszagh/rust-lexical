extern crate lexical_core;

#[no_mangle]
#[inline(never)]
pub fn parse_u8(data: &str) -> lexical_core::Result<u8> {
    lexical_core::parse::<u8>(data.as_bytes())
}

#[no_mangle]
#[inline(never)]
pub fn parse_u16(data: &str) -> lexical_core::Result<u16> {
    lexical_core::parse::<u16>(data.as_bytes())
}

//#[no_mangle]
//pub fn parse_u32(data: &str) -> Option<u32> {
//    data.parse()
//}
//
//#[no_mangle]
//pub fn parse_u64(data: &str) -> Option<u64> {
//    data.parse()
//}
//
//#[no_mangle]
//pub fn parse_u128(data: &str) -> Option<u12> {
//    data.parse()
//}
//
//#[no_mangle]
//pub fn parse_usize(data: &str) -> Option<usi> {
//    data.parse()
//}
//
//#[no_mangle]
//pub fn parse_i8(data: &str) -> Option<i8,> {
//    data.parse()
//}
//
//#[no_mangle]
//pub fn parse_i16(data: &str) -> Option<i16> {
//    data.parse()
//}
//
//#[no_mangle]
//pub fn parse_i32(data: &str) -> Option<i32> {
//    data.parse()
//}
//
//#[no_mangle]
//pub fn parse_i64(data: &str) -> Option<i64> {
//    data.parse()
//}
//
//#[no_mangle]
//pub fn parse_i128(data: &str) -> Option<i12> {
//    data.parse()
//}
//
//#[no_mangle]
//pub fn parse_isize(data: &str) -> Option<isi> {
//    data.parse()
//}


pub fn main() {
    println!("{}", parse_u8("0").unwrap());
    println!("{}", parse_u16("0").unwrap());
    //println!("{}", parse_u32("0").unwrap());
    //println!("{}", parse_u64("0").unwrap());
    //println!("{}", parse_u128("0").unwrap());
    //println!("{}", parse_usize("0").unwrap());
    //println!("{}", parse_i8("0").unwrap());
    //println!("{}", parse_i16("0").unwrap());
    //println!("{}", parse_i32("0").unwrap());
    //println!("{}", parse_i64("0").unwrap());
    //println!("{}", parse_i128("0").unwrap());
    //println!("{}", parse_isize("0").unwrap());
}
