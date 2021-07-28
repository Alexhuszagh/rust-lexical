use lexical_util::format::NumberFormatBuilder;

pub const fn to_format(radix: u32) -> u128 {
    NumberFormatBuilder::new()
        .mantissa_radix(radix as u8)
        .build()
}
